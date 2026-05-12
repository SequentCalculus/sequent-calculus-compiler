//! This module defines programs in Core.

use printer::*;
use std::collections::{HashMap, HashSet};

use crate::syntax::*;
use crate::typing::inference::{VarNameGenerator, constraint_unification};
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
    
    /// the main function for type inference. It consumes the (uncheckd)[`Program`] and returns a [`CheckedProgram`] with
    /// the all types inferred and overloading resolved.
    pub fn inference_types(self) -> Result<CheckedProgram, Error>{
        let mut symbol_table = build_symbol_table(&self)?;
        let var_name_generator = &mut VarNameGenerator::new();
        let mut constraints = Vec::new();

        let mut data_types = Vec::new();
        let mut codata_types = Vec::new();
        let mut defs = Vec::new();
        let mut overloaded_defs_counter = HashMap::new();
        
        for decl in self.declarations {
            match decl {
                Declaration::Data(data) => {
                    data.check(&symbol_table)?;
                    data_types.push(data);
                }
                Declaration::Codata(codata) => {
                    codata.check(&symbol_table)?;
                    codata_types.push(codata);
                }
                Declaration::Def(mut def) => {
                    constraints.append(&mut def.constraint_equations(&mut symbol_table, var_name_generator)?);

                    // the names of overloaded functions are replaced with a unique name.
                    if symbol_table.variational_defs[&def.name].len() > 1 {

                        // the name consists of the index of the definition, so they are counted in the overloaded defs counter
                        if let Some(counter) = overloaded_defs_counter.get_mut(&def.name) {
                            def.name = symbol_table::build_unique_def_name(&def.name, counter);
                            *counter += 1;
                        } else {
                            def.name = symbol_table::build_unique_def_name(&def.name, &0);
                            overloaded_defs_counter.insert(def.name.clone(), 1);
                        }
                    }
                    defs.push(def);
                }
            }
        }

        let (solutions, conflicts) = constraint_unification(constraints);

        let all_possible_choices = symbol_table.variational_defs.iter().map(|(name, variation_list)| (name.clone(), variation_list.len()))
            .filter(|(_, size)| *size > 1 ).collect();
        
        let selected_world = crate::typing::world_resolution::resolve_worlds(&all_possible_choices, conflicts)?;

        let choices_map: HashMap<Name, usize> = selected_world.iter().cloned().collect();

        // now all solutions that are part of the selected world are filtered.
        let mut selected_solutions = solutions;
        for (name, wanted_id) in selected_world {
            selected_solutions.retain(|s| match s.choices.get(&name) {
                Some(id) => wanted_id == *id,

                // if the solution doesn't have a choice for the wanted name, it is invariant to the choice. So it is part of the world
                None => true
            });
        }

        // the solutions are converted to a HashMap and then they are inserted in the program
        let mut type_mapping: HashMap<String, Ty> = selected_solutions.into_iter().map(crate::typing::inference::Solution::get_only_solution).collect();

        // the mapping is applied on it self. The mapping can contain a reference to another type variable.
        let reference_map = type_mapping.clone();
        for (_, ty) in type_mapping.iter_mut() {
            while !ty.collect_var_names().is_empty() {
                ty.mut_subst_ty(&reference_map);
            }
        }

        for def in &mut defs {
            def.insert_inferred_type(&type_mapping, &mut symbol_table, &choices_map)?;
        }

        Ok(CheckedProgram { data_types, codata_types, defs })
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

fn dummy_world_selector(_conflicts: Vec<crate::typing::inference::IncompatibleChoices>) -> Vec<(Name, usize)> {
    vec![("a".to_string(), 4), ("b".to_string(), 7)]
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
