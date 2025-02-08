use std::collections::HashSet;

use crate::parser::util::ToMiette;
use codespan::Span;

use printer::{DocAllocator, Print};

use crate::{
    syntax::{context::TypeContext, Name},
    typing::{
        errors::Error,
        symbol_table::{build_symbol_table, Polarity, SymbolTable},
    },
};

pub mod codata_declaration;
pub mod data_declaration;
pub mod definition;
pub use codata_declaration::*;
pub use data_declaration::*;
pub use definition::*;

// Declaration
//
//

/// A top-level declaration in a module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Definition(Definition),
    DataDeclaration(DataDeclaration),
    CodataDeclaration(CodataDeclaration),
}

impl Print for Declaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Declaration::Definition(definition) => definition.print(cfg, alloc),
            Declaration::DataDeclaration(data_declaration) => data_declaration.print(cfg, alloc),
            Declaration::CodataDeclaration(codata_declaration) => {
                codata_declaration.print(cfg, alloc)
            }
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
    pub defs: Vec<Definition>,
    pub data_types: Vec<DataDeclaration>,
    pub codata_types: Vec<CodataDeclaration>,
}

impl Module {
    pub fn check(self) -> Result<CheckedModule, Error> {
        let mut symbol_table = build_symbol_table(&self)?;
        self.check_with_table(&mut symbol_table)
    }

    fn check_with_table(self, symbol_table: &mut SymbolTable) -> Result<CheckedModule, Error> {
        let mut defs = Vec::new();
        for decl in self.declarations {
            match decl {
                Declaration::Definition(definition) => {
                    defs.push(definition);
                }
                Declaration::DataDeclaration(data_declaration) => {
                    data_declaration.check(symbol_table)?;
                }
                Declaration::CodataDeclaration(codata_declaration) => {
                    codata_declaration.check(symbol_table)?;
                }
            }
        }

        let defs = defs
            .into_iter()
            .map(|def| def.check(symbol_table))
            .collect::<Result<_, Error>>()?;

        let mut data_types = Vec::new();
        let mut codata_types = Vec::new();
        for (name, (pol, type_args, xtors)) in &symbol_table.types {
            match pol {
                Polarity::Data => {
                    let ctors = xtors
                        .iter()
                        .map(|name| {
                            let Some(args) = symbol_table
                                .ctors
                                .get(&(name.clone() + &type_args.print_to_string(None)))
                            else {
                                return Err(Error::Undefined {
                                    span: Span::default().to_miette(),
                                    name: name.clone(),
                                });
                            };
                            Ok(CtorSig {
                                span: Span::default(),
                                name: name.clone(),
                                args: args.clone(),
                            })
                        })
                        .collect::<Result<_, _>>()?;
                    let declaration = DataDeclaration {
                        span: Span::default(),
                        name: name.clone(),
                        type_params: TypeContext::default(),
                        ctors,
                    };
                    data_types.push(declaration);
                }
                Polarity::Codata => {
                    let dtors = xtors
                        .iter()
                        .map(|name| {
                            let Some((args, cont_ty)) = symbol_table
                                .dtors
                                .get(&(name.clone() + &type_args.print_to_string(None)))
                            else {
                                return Err(Error::Undefined {
                                    span: Span::default().to_miette(),
                                    name: name.clone(),
                                });
                            };
                            Ok(DtorSig {
                                span: Span::default(),
                                name: name.clone(),
                                args: args.clone(),
                                cont_ty: cont_ty.clone(),
                            })
                        })
                        .collect::<Result<_, _>>()?;
                    let declaration = CodataDeclaration {
                        span: Span::default(),
                        name: name.clone(),
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
}

impl Module {
    pub fn data_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for decl in &self.declarations {
            if let Declaration::DataDeclaration(data) = decl {
                names.insert(data.name.clone());
            }
        }

        names
    }

    pub fn codata_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for decl in &self.declarations {
            if let Declaration::CodataDeclaration(codata) = decl {
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

        let decls = self.declarations.iter().map(|decl| decl.print(cfg, alloc));

        alloc.intersperse(decls, sep)
    }
}

#[cfg(test)]
mod module_tests {
    use codespan::Span;
    use printer::Print;

    use super::{Definition, Module};
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
            declarations: vec![Definition {
                span: Span::default(),
                name: "x".to_string(),
                context: TypingContext::default(),
                body: Term::Lit(Lit::mk(4)),
                ret_ty: Ty::mk_i64(),
            }
            .into()],
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
            declarations: vec![Definition {
                span: Span::default(),
                name: "f".to_string(),
                context: ctx,
                body: Term::Lit(Lit::mk(4)),
                ret_ty: Ty::mk_i64(),
            }
            .into()],
        }
    }

    #[test]
    fn display_args() {
        assert_eq!(
            example_args().print_to_string(Default::default()),
            "def f(x: i64, a :cns i64): i64 { 4 }".to_string(),
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
        let d1 = Definition {
            span: Span::default(),
            name: "f".to_string(),
            context: TypingContext::default(),
            body: Term::Lit(Lit::mk(2)),
            ret_ty: Ty::mk_i64(),
        };

        let d2 = Definition {
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
