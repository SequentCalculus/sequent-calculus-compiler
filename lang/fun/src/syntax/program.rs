//! This module defines programs in Core.

use printer::*;
use std::collections::{HashMap, HashSet};

use crate::syntax::*;
use crate::typing::inference::{TypeInferenceState, solve_inference_constraints};
use crate::typing::*;

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
    /// This function typechecks a program by type inference. It consumes the unchecked
    /// [`Program`] and returns a [`CheckedProgram`] with all types inferred and overloading
    /// resolved.
    pub fn check(self) -> Result<CheckedProgram, Error> {
        let mut state = TypeInferenceState::new(build_symbol_table(&self)?);

        let mut defs = Vec::new();
        for decl in self.declarations {
            match decl {
                Declaration::Data(data) => {
                    data.check(&state.symbol_table)?;
                }
                Declaration::Codata(codata) => {
                    codata.check(&state.symbol_table)?;
                }
                Declaration::Def(mut def) => {
                    def.collect_constraints(&mut state)?;
                    defs.push(def);
                }
            }
        }

        rename_overloaded_defs(&mut defs, &state.symbol_table);

        // recovering the properties from the TypeInferenceState
        let TypeInferenceState {
            mut symbol_table,
            constraints,
            choices,
            ..
        } = state;

        let (type_vars, world) = solve_inference_constraints(constraints, &choices)?;

        for def in &mut defs {
            def.insert_inferred_types(&type_vars, &mut symbol_table, &world)?;
        }

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
                                span: None,
                                // keep base name for xtor in all instances
                                name: base_name,
                                args,
                            }
                        })
                        .collect();
                    let declaration = Data {
                        span: None,
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
                                span: None,
                                // keep base name for xtor in all instances
                                name: base_name,
                                args,
                                cont_ty,
                            }
                        })
                        .collect();
                    let declaration = Codata {
                        span: None,
                        name,
                        type_params: TypeContext::default(),
                        dtors,
                    };
                    codata_types.push(declaration);
                }
            }
        }

        Ok(CheckedProgram {
            data_types,
            codata_types,
            defs,
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

/// This function renames all overloaded definitions to the unique names of their overloads (see
/// [`overload_name`](symbol_table::overload_name)). The index of an overload is the position of its
/// signature in the symbol table, which records the signatures in the order of the declarations.
fn rename_overloaded_defs(defs: &mut [Def], symbol_table: &SymbolTable) {
    let mut next_index: HashMap<Name, usize> = HashMap::new();
    for def in defs {
        if symbol_table.defs[&def.name].len() > 1 {
            let index = next_index.entry(def.name.clone()).or_insert(0);
            def.name = symbol_table::overload_name(&def.name, index);
            *index += 1;
        }
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

impl Print for CheckedProgram {
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

        let datas = self.data_types.iter().map(|decl| decl.print(cfg, alloc));
        let codatas = self.codata_types.iter().map(|decl| decl.print(cfg, alloc));
        let definitions = self.defs.iter().map(|decl| decl.print(cfg, alloc));

        let declarations = datas.chain(codatas).chain(definitions);

        alloc.intersperse(declarations, sep)
    }
}

#[cfg(test)]
mod program_tests {
    use printer::Print;

    use crate::{
        parser::fun,
        syntax::{
            context::TypingContext,
            declarations::Def,
            program::Program,
            terms::{Lit, Term},
            types::Ty,
            util::dummy_span,
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
                    span: dummy_span(),
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
                    span: dummy_span(),
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
            span: dummy_span(),
            name: "f".to_string(),
            context: TypingContext::default(),
            body: Term::Lit(Lit::mk(2)),
            ret_ty: Ty::mk_i64(),
        };

        let d2 = Def {
            span: dummy_span(),
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
