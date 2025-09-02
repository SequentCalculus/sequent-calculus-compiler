//! This module defines programs in Core.

use std::collections::HashSet;

use codespan::Span;

use printer::{DocAllocator, Print};

use crate::{
    syntax::{
        context::TypeContext,
        declarations::{
            Declaration, Polarity,
            codata::{Codata, DtorSig},
            data::{CtorSig, Data},
            def::Def,
        },
        names::Name,
    },
    typing::{
        errors::Error,
        symbol_table::{SymbolTable, build_symbol_table},
    },
};

/// This struct defines a module consisting of a list of [`Declaration`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

/// This struct defines a typechecked module created from a [`Program`] by checking each contained
/// [`Declaration`]. The checked module only contans monomorphic instances of data and codata types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProgram {
    /// Checked data type instances
    pub data_types: Vec<Data>,
    /// Checked codata type instances
    pub codata_types: Vec<Codata>,
    /// Checked top-level functions
    pub defs: Vec<Def>,
}

impl Program {
    /// This function typechecks all declarations in a module, creating a checked module with
    /// monomorphic type instances.
    pub fn check(self) -> Result<CheckedProgram, Error> {
        let symbol_table = build_symbol_table(&self)?;
        self.check_with_table(symbol_table)
    }

    /// This function typechecks a module, creating a checked module with monomorphic type
    /// instances, with given symbol table.
    fn check_with_table(self, mut symbol_table: SymbolTable) -> Result<CheckedProgram, Error> {
        let mut defs = Vec::new();
        // we check the well-formedness of type declarations first
        for decl in self.declarations {
            match decl {
                Declaration::Data(data) => {
                    data.check(&symbol_table)?;
                }
                Declaration::Codata(codata) => {
                    codata.check(&symbol_table)?;
                }
                Declaration::Def(def) => {
                    defs.push(def);
                }
            }
        }

        let defs = defs
            .into_iter()
            .map(|def| def.check(&mut symbol_table))
            .collect::<Result<_, Error>>()?;

        // collect all instances of type templates from the symbol table
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
                                // keep base name for xtor in all instances
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
                                // keep base name for xtor in all instances
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

        Ok(CheckedProgram {
            defs,
            data_types,
            codata_types,
        })
    }

    /// This function returns the names of all data type templates in a module.
    pub fn data_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for declaration in &self.declarations {
            if let Declaration::Data(data) = declaration {
                names.insert(data.name.clone());
            }
        }

        names
    }

    /// This function returns the names of all codata type templates in a module.
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

impl Print for Program {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep`
        // option is set. This is useful for typesetting examples in papers which have to make
        // economic use of vertical space.
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
mod program_tests {
    use codespan::Span;
    use printer::Print;

    use crate::{
        parser::fun,
        syntax::{
            context::TypingContext,
            declarations::Def,
            program::Program,
            terms::{Lit, Term},
            types::Ty,
        },
    };
    use std::collections::HashSet;

    // Program with one definition without arguments
    //
    //

    fn example_simple() -> Program {
        Program {
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
            "def x(): i64 {\n    4\n}".to_string()
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

    fn example_args() -> Program {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_covar("a", Ty::mk_i64());
        Program {
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
            "def f(x: i64, a: cns i64): i64 {\n    4\n}".to_string(),
        )
    }

    #[test]
    fn parse_args() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(x: i64, a:cns i64): i64 {\n    4\n}"),
            Ok(example_args().into())
        )
    }

    // Program with two definitions
    //
    //

    fn example_two() -> Program {
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
        Program {
            declarations: vec![d1.into(), d2.into()],
        }
    }

    #[test]
    fn display_two() {
        assert_eq!(
            example_two().print_to_string(Default::default()),
            "def f(): i64 {\n    2\n}\n\ndef g(): i64 {\n    4\n}".to_string(),
        )
    }

    #[test]
    fn parse_two() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(): i64 { 2 }\n def g(): i64 { 4 }"),
            Ok(example_two().into())
        )
    }
}
