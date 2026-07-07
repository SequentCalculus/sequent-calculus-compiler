//! This module defines programs in Core.

use printer::*;
use std::collections::{HashMap, HashSet};

use crate::syntax::*;
use crate::typing::inference::{ConstraintBank, constraint_unification};
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

impl CheckedProgram {
    pub fn print_all(&mut self) {
        self.data_types.sort_by_key(|data| data.name.clone());
        println!("Data:");
        for data in self.data_types.iter() {
            println!("\t{:?}", data);
        }

        self.codata_types.sort_by_key(|codata| codata.name.clone());
        println!("Codata:");
        for codata in self.codata_types.iter() {
            println!("\t{:?}", codata);
        }

        self.defs.sort_by_key(|defs| defs.name.clone());
        println!("Defs:");
        for def in self.defs.iter() {
            println!("\t{:?}", def);
        }
    }
}

impl Program {
    /// the main function for type inference. It consumes the (uncheckd)[`Program`] and returns a [`CheckedProgram`] with
    /// the all types inferred and overloading resolved.
    pub fn inference_types(self) -> Result<CheckedProgram, Error> {
        let mut constraint_bank = ConstraintBank {
            symbol_table: build_symbol_table(&self)?,
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        let mut defs = Vec::new();
        let mut overloaded_defs_counter = HashMap::new();

        for decl in self.declarations {
            match decl {
                Declaration::Data(data) => {
                    data.check(&constraint_bank.symbol_table)?;
                }
                Declaration::Codata(codata) => {
                    codata.check(&constraint_bank.symbol_table)?;
                }
                Declaration::Def(mut def) => {
                    def.gather_constraints(&mut constraint_bank)?;

                    // the names of overloaded functions are replaced with a unique name.
                    if constraint_bank.symbol_table.variational_defs[&def.name].len() > 1 {
                        // the name consists of the index of the definition, so they are counted in the overloaded defs counter
                        if let Some(counter) = overloaded_defs_counter.get_mut(&def.name) {
                            def.name = symbol_table::build_unique_def_name(&def.name, counter);
                            *counter += 1;
                        } else {
                            overloaded_defs_counter.insert(def.name.clone(), 1);
                            def.name = symbol_table::build_unique_def_name(&def.name, &0);
                        }
                    }
                    defs.push(def);
                }
            }
        }

        // recovering the properties from the ConstraintBank
        let ConstraintBank {
            mut symbol_table,
            constraints,
            possible_choices,
            ..
        } = constraint_bank;

        let (solutions, conflicts) = constraint_unification(constraints);

        // if there are no choices, world resolving is skipped
        let selected_world = if possible_choices.len() > 0 {
            crate::typing::world_resolution::resolve_worlds(&possible_choices, conflicts)?
        } else if conflicts.len() > 0 {
            // there is only one world, but there are also conflicts. So there is no solution
            // todo!("Better Error")
            return Err(conflicts[0].error.clone());
        } else {
            Vec::new()
        };

        let choices_map: HashMap<u32, usize> = selected_world.iter().cloned().collect();

        // now all solutions that are part of the selected world are filtered.
        let mut selected_solutions = solutions;
        for (choice_id, signature_id) in selected_world {
            selected_solutions.retain(|s| match s.choices.get(&choice_id) {
                Some(id) => signature_id == *id,

                // if the solution doesn't have a choice for the wanted name, it is invariant to the choice. So it is part of the world
                None => true,
            });
        }

        let mut type_mapping: HashMap<String, Ty> = HashMap::new();

        /*
        There could be several solutions for one type var.
        The best solution is chosen
        This "best" is the solution with the least type vars in the ty.
        There could also be several solutions with different choice annotations
        but they are non conflicting, since this would be found by the unification.
        */
        for solution in selected_solutions {
            if let Some(subst_ty) = type_mapping.get_mut(&solution.var_name) {
                if solution.ty.collect_var_names().len() < subst_ty.collect_var_names().len() {
                    *subst_ty = solution.ty;
                }
            } else {
                type_mapping.insert(solution.var_name, solution.ty);
            }
        }

        // the type mapping is applied on it self, to get the complete transitive hull
        let reference_mapping = type_mapping.clone();

        for (_, ty) in type_mapping.iter_mut() {
            loop {
                let var_names = ty.collect_var_names();

                if var_names.is_empty() {
                    break;
                }

                if var_names.iter().any(|s| !reference_mapping.contains_key(s)) {
                    let missing_names: Vec<String> = ty
                        .collect_var_names()
                        .into_iter()
                        .filter(|k| !reference_mapping.contains_key(k))
                        .collect();
                    panic!(
                        "Missing type var names in the final type mapping: {:?}",
                        missing_names
                    );
                }

                ty.mut_subst_ty(&reference_mapping);
            }
        }

        for def in &mut defs {
            def.insert_inferred_type(&type_mapping, &mut symbol_table, &choices_map)?;
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
