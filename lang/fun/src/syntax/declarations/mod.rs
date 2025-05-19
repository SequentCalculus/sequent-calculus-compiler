use std::collections::HashSet;

use codespan::Span;

use printer::{DocAllocator, Print};

use crate::{
    syntax::{Name, context::TypeContext},
    typing::{
        errors::Error,
        symbol_table::{SymbolTable, build_symbol_table},
    },
};

pub mod codata;
pub mod data;
pub mod def;
pub use codata::*;
pub use data::*;
pub use def::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Polarity {
    Data,
    Codata,
}

// Declaration
//
//

// TODO: contemplate boxing large variants here
#[allow(clippy::large_enum_variant)]
/// A top-level declaration in a module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Def(Def),
    Data(Data),
    Codata(Codata),
}

impl Print for Declaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Declaration::Def(def) => def.print(cfg, alloc),
            Declaration::Data(data) => data.print(cfg, alloc),
            Declaration::Codata(codata) => codata.print(cfg, alloc),
        }
    }
}

// Module
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedModule {
    pub defs: Vec<Def>,
    pub data_types: Vec<Data>,
    pub codata_types: Vec<Codata>,
}

impl Module {
    pub fn check(self) -> Result<CheckedModule, Error> {
        let symbol_table = build_symbol_table(&self)?;
        self.check_with_table(symbol_table)
    }

    fn check_with_table(self, mut symbol_table: SymbolTable) -> Result<CheckedModule, Error> {
        let mut defs = Vec::new();
        for decl in self.declarations {
            match decl {
                Declaration::Def(def) => {
                    defs.push(def);
                }
                Declaration::Data(data) => {
                    data.check(&symbol_table)?;
                }
                Declaration::Codata(codata) => {
                    codata.check(&symbol_table)?;
                }
            }
        }

        let defs = defs
            .into_iter()
            .map(|def| def.check(&mut symbol_table))
            .collect::<Result<_, Error>>()?;

        // collect all instances of type templates
        let mut data_types = Vec::new();
        let mut codata_types = Vec::new();
        for (name, (pol, type_args, xtors)) in symbol_table.types {
            match pol {
                Polarity::Data => {
                    let ctors = xtors
                        .into_iter()
                        .map(|base_name| {
                            let full_name = base_name.clone() + &type_args.print_to_string(None);
                            let args = symbol_table
                                .ctors
                                .get(&full_name)
                                .unwrap_or_else(|| {
                                    panic!("Couldn't find constructor {full_name} in symbol_table.")
                                })
                                .clone();
                            CtorSig {
                                span: Span::default(),
                                name: base_name,
                                args,
                            }
                        })
                        .collect();
                    let declaration = Data {
                        span: Span::default(),
                        name,
                        type_params: TypeContext::default(),
                        ctors,
                    };
                    data_types.push(declaration);
                }
                Polarity::Codata => {
                    let dtors = xtors
                        .into_iter()
                        .map(|base_name| {
                            let full_name = base_name.clone() + &type_args.print_to_string(None);
                            let (args, cont_ty) = symbol_table
                                .dtors
                                .get(&full_name)
                                .unwrap_or_else(|| {
                                    panic!("Couldn't find destructor {full_name} in symbol_table.")
                                })
                                .clone();
                            DtorSig {
                                span: Span::default(),
                                name: base_name,
                                args,
                                cont_ty,
                            }
                        })
                        .collect();
                    let declaration = Codata {
                        span: Span::default(),
                        name,
                        type_params: TypeContext::default(),
                        dtors,
                    };
                    codata_types.push(declaration);
                }
            }
        }

        Ok(CheckedModule {
            defs,
            data_types,
            codata_types,
        })
    }

    pub fn data_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for declaration in &self.declarations {
            if let Declaration::Data(data) = declaration {
                names.insert(data.name.clone());
            }
        }

        names
    }

    pub fn codata_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for declaration in &self.declarations {
            if let Declaration::Codata(codata) = declaration {
                names.insert(codata.name.clone());
            }
        }
        names
    }
}

impl Print for Module {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep` option is set.
        // This is useful for typesetting examples in papers which have to make economic use of vertical space.
        let sep = if cfg.omit_decl_sep {
            alloc.line()
        } else {
            alloc.line().append(alloc.line())
        };

        let declarations = self.declarations.iter().map(|decl| decl.print(cfg, alloc));

        alloc.intersperse(declarations, sep)
    }
}

#[cfg(test)]
mod module_tests {
    use codespan::Span;
    use printer::Print;

    use super::{Def, Module};
    use crate::{
        parser::fun,
        syntax::{
            context::TypingContext,
            terms::{Lit, Term},
            types::Ty,
        },
    };
    use std::collections::HashSet;

    // Program with one definition without arguments
    //
    //

    fn example_simple() -> Module {
        Module {
            declarations: vec![
                Def {
                    span: Span::default(),
                    name: "x".to_string(),
                    context: TypingContext::default(),
                    body: Term::Lit(Lit::mk(4)),
                    ret_ty: Ty::mk_i64(),
                }
                .into(),
            ],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            example_simple().print_to_string(Default::default()),
            "def x: i64 { 4 }".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def x: i64 { 4 }"),
            Ok(example_simple().into())
        );
    }

    #[test]
    fn data_simple() {
        let result = example_simple().data_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn codata_simple() {
        let result = example_simple().codata_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    // Program with one definition which takes arguments
    //
    //

    fn example_args() -> Module {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_covar("a", Ty::mk_i64());
        Module {
            declarations: vec![
                Def {
                    span: Span::default(),
                    name: "f".to_string(),
                    context: ctx,
                    body: Term::Lit(Lit::mk(4)),
                    ret_ty: Ty::mk_i64(),
                }
                .into(),
            ],
        }
    }

    #[test]
    fn display_args() {
        assert_eq!(
            example_args().print_to_string(Default::default()),
            "def f(x : i64, a :cns i64): i64 { 4 }".to_string(),
        )
    }

    #[test]
    fn parse_args() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(x: i64, a :cns i64) : i64 { 4 }"),
            Ok(example_args().into())
        )
    }

    // Program with two definitions
    //
    //

    fn example_two() -> Module {
        let d1 = Def {
            span: Span::default(),
            name: "f".to_string(),
            context: TypingContext::default(),
            body: Term::Lit(Lit::mk(2)),
            ret_ty: Ty::mk_i64(),
        };

        let d2 = Def {
            span: Span::default(),
            name: "g".to_string(),
            context: TypingContext::default(),
            body: Term::Lit(Lit::mk(4)),
            ret_ty: Ty::mk_i64(),
        };
        Module {
            declarations: vec![d1.into(), d2.into()],
        }
    }

    #[test]
    fn display_two() {
        assert_eq!(
            example_two().print_to_string(Default::default()),
            "def f: i64 { 2 }\n\ndef g: i64 { 4 }".to_string(),
        )
    }

    #[test]
    fn parse_two() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f() : i64 { 2 }\n def g() : i64 { 4 }"),
            Ok(example_two().into())
        )
    }
}
