//! This module defines types in Core.

use printer::tokens::I64;
use printer::*;

use crate::mono::constraints::{ConstraintCollector, FlowConstraintSet, collect_type_flow};
use crate::mono::erasure::erase_ty;
use crate::mono::errors::MonoError;
use crate::mono::specialize::{Specialize, SpecializeContext};
use crate::splitting::rewrite::Rewrite;
use crate::splitting::split_table::SplitTable;
use crate::typing::check::{Checked, check_arity};
use crate::typing::env::GlobalEnv;
use crate::typing::errors::{LocatedTypeError, TypeError};
use crate::{bail, syntax::*};

/// This enum encodes the types of AxCut. They are either integers or names of user-declared types.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum Ty {
    /// Signed 64-Bit integer.
    I64,
    /// User-declared data or codata type with additional type arguments.
    Decl {
        name: Identifier,
        type_args: TypeArgs,
    },
    /// Type variable, used for type parameters.
    Var(Identifier),
}

impl Ty {
    /// This function checks whether a type is a codata type.
    /// - `codata_types` is the list of codata type declarations in the program.
    /// - `type_params` is the ambient list of declaration-site type parameters currently in
    ///   scope (with their declared polarity), used to resolve `Ty::Var`. A `Ty::Var` not found
    ///   in this list indicates a checker bug (an out-of-scope type variable should never reach
    ///   this point) and is reported loudly via `unreachable!` rather than silently defaulting.
    pub fn is_codata(&self, codata_types: &[CodataDeclaration], type_params: &[TypeParam]) -> bool {
        match self {
            Ty::I64 => false,
            Ty::Decl { name, .. } => codata_types
                .iter()
                .any(|declaration| declaration.name == *name),
            Ty::Var(param) => {
                let declared = type_params
                    .iter()
                    .find(|type_param| *type_param == param)
                    .unwrap_or_else(|| {
                        unreachable!(
                            "Ty::Var {} not found among the ambient type parameters {:?} \
                             passed to is_codata",
                            param.print_to_string(None),
                            type_params
                        )
                    });
                matches!(declared.polarity, ParamPolarity::Codata)
            }
        }
    }

    /// This function substitutes type variables in a type with their corresponding concrete types according to the provided substitution mapping.
    /// - `subst` is an optional tuple containing a reference to the list of type parameters and their corresponding concrete types for the current substitution context.
    pub fn substitute(&self, subst: (&[Identifier], &[Ty])) -> Self {
        match self {
            Ty::I64 => Ty::I64,
            Ty::Var(param) => {
                let (params, args) = subst;
                if let Some(pos) = params.iter().position(|p| p == param) {
                    if let Some(concrete_ty) = args.get(pos) {
                        return concrete_ty.clone();
                    }
                }
                Ty::Var(param.clone())
            }
            Ty::Decl { name, type_args } => Ty::Decl {
                name: name.clone(),
                type_args: TypeArgs {
                    args: type_args.args.iter().map(|a| a.substitute(subst)).collect(),
                },
            },
        }
    }
}

impl Checked for Ty {
    fn check(
        &self,
        type_params: &[Identifier],
        _context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        match self {
            Ty::I64 => Ok(()),
            Ty::Var(param) => {
                // check that the type variable is declared as a type parameter in the current context
                if type_params.iter().any(|type_param| type_param == param) {
                    Ok(())
                } else {
                    bail!(TypeError::UndeclaredType(param.print_to_string(None)))
                }
            }
            Ty::Decl { name, type_args } => {
                // check that the type name is declared as a data or codata type and get the type params
                let Some(declaration_type_params) = env.lookup_type_params(name) else {
                    bail!(TypeError::UndeclaredType(name.print_to_string(None)))
                };

                // check that the number of type arguments matches the number of type parameters in the declaration
                check_arity(declaration_type_params.len(), type_args.args.len())?;

                // check that all type arguments are well-formed
                for arg in &type_args.args {
                    arg.check(type_params, _context, env)?;
                }
                Ok(())
            }
        }
    }
}

impl ConstraintCollector for Ty {
    fn collect_constraints(&self, env: &GlobalEnv) -> Result<FlowConstraintSet, MonoError> {
        match self {
            Ty::I64 => Ok(FlowConstraintSet::new()),
            Ty::Var(_) => Ok(FlowConstraintSet::new()),
            Ty::Decl { name, type_args } => {
                // Self is a `Ty::Decl` here, so `is_codata`'s `Ty::Var` branch (the only one that
                // consults `type_params`) can never be reached - passing an empty ambient list is
                // safe.
                if self.is_codata(env.codata_decls, &[]) {
                    let Some(template) = env.lookup_codata_decl(name) else {
                        return Err(MonoError::UndeclaredType(name.print_to_string(None)));
                    };

                    let template_ids: Vec<Identifier> =
                        template.type_params.iter().map(|p| p.id.clone()).collect();
                    collect_type_flow(&type_args.args, &template_ids)
                } else {
                    let Some(template) = env.lookup_data_decl(name) else {
                        return Err(MonoError::UndeclaredType(name.print_to_string(None)));
                    };

                    let template_ids: Vec<Identifier> =
                        template.type_params.iter().map(|p| p.id.clone()).collect();
                    collect_type_flow(&type_args.args, &template_ids)
                }
            }
        }
    }
}

impl Print for Ty {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            Ty::I64 => alloc.typ(I64),
            Ty::Decl { name, type_args } => {
                alloc.typ(&name.name).append(type_args.print(cfg, alloc))
            }
            Ty::Var(name) => alloc.typ(&name.print_to_string(None)),
        }
    }
}

impl Specialize for Ty {
    fn specialize(&self, context: &SpecializeContext) -> Self {
        match self {
            Ty::I64 => Ty::I64,

            Ty::Var(param) => {
                let (params, args) = &context.subst;
                let pos = params.iter().position(|p| p == param).unwrap_or_else(|| {
                    panic!(
                        "type variable {} not found in substitution",
                        param.print_to_string(None)
                    )
                });
                args[pos].specialize(&SpecializeContext::ground(
                    context.table,
                    context.erased_decls,
                ))
            }

            Ty::Decl { name, type_args } => {
                let substituted: Vec<Ty> = type_args
                    .args
                    .iter()
                    .map(|a| a.substitute((&context.subst.0, &context.subst.1)))
                    .collect();
                let candidate = Ty::Decl {
                    name: name.clone(),
                    type_args: TypeArgs { args: substituted },
                };

                // Erase the type inline to match the right candidate for the lookup
                let erased_candidate = erase_ty(&candidate, &context.erased_decls.0);
                let Ty::Decl {
                    name: erased_name,
                    type_args: erased_args,
                } = &erased_candidate
                else {
                    unreachable!("erase_ty always preserves the Decl variant");
                };
                let mangled = context.table.lookup(erased_name, &erased_args.args).clone();
                Ty::Decl {
                    name: mangled,
                    type_args: TypeArgs::default(),
                }
            }
        }
    }
}

impl Rewrite for Ty {
    fn rewrite(&self, table: &SplitTable) -> Self {
        match self {
            Ty::I64 => Ty::I64,
            Ty::Var(v) => Ty::Var(v.clone()),
            Ty::Decl { name, type_args } => Ty::Decl {
                name: table.resolve_ty_name(name).clone(),
                type_args: type_args.rewrite(table),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Default)]
pub struct TypeArgs {
    /// The type arguments
    pub args: Vec<Ty>,
}

impl Rewrite for TypeArgs {
    fn rewrite(&self, table: &SplitTable) -> Self {
        TypeArgs {
            args: self.args.rewrite(table),
        }
    }
}

impl ConstraintCollector for TypeArgs {
    fn collect_constraints(&self, env: &GlobalEnv) -> Result<FlowConstraintSet, MonoError> {
        let mut constraints = FlowConstraintSet::new();
        for arg in &self.args {
            constraints.extend(arg.collect_constraints(env)?);
        }
        Ok(constraints)
    }
}

impl Print for TypeArgs {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        if self.args.is_empty() {
            alloc.nil()
        } else {
            let mut args_printed = alloc.nil();
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    args_printed = args_printed.append(alloc.text(", "));
                }
                args_printed = args_printed.append(arg.print(cfg, alloc));
            }
            args_printed.brackets()
        }
    }
}

#[cfg(test)]
mod type_tests {
    use super::{Identifier, Ty, TypeArgs};
    use printer::Print;

    #[test]
    fn display_i64() {
        assert_eq!(Ty::I64.print_to_string(None), "i64");
    }

    #[test]
    fn display_decl() {
        let ty = Ty::Decl {
            name: Identifier::new("List".to_string()),
            type_args: TypeArgs {
                args: vec![Ty::I64],
            },
        };
        assert_eq!(ty.print_to_string(None), "List[i64]");
    }

    #[test]
    fn display_decl2() {
        let ty = Ty::Decl {
            name: Identifier::new("List".to_string()),
            type_args: TypeArgs {
                args: vec![Ty::Var(Identifier {
                    name: "A".to_string(),
                    id: 1,
                })],
            },
        };
        assert_eq!(ty.print_to_string(None), "List[A_1]");
    }

    #[test]
    fn is_codata_resolves_ty_var_via_ambient_type_params() {
        use crate::syntax::type_params::{ParamPolarity, TypeParam};

        let positive = Identifier {
            name: "A".to_string(),
            id: 1,
        };
        let negative = Identifier {
            name: "B".to_string(),
            id: 2,
        };
        let type_params = vec![
            TypeParam {
                id: positive.clone(),
                polarity: ParamPolarity::Data,
            },
            TypeParam {
                id: negative.clone(),
                polarity: ParamPolarity::Codata,
            },
        ];

        assert!(!Ty::Var(positive).is_codata(&[], &type_params));
        assert!(Ty::Var(negative).is_codata(&[], &type_params));
    }

    #[test]
    #[should_panic(expected = "not found among the ambient type parameters")]
    fn is_codata_panics_loudly_on_out_of_scope_ty_var() {
        // An out-of-scope `Ty::Var` reaching `is_codata` indicates a checker bug and must be
        // reported loudly (this is the direct fix for the polarity-annotation bug: silently
        // defaulting to `false` here previously masked exactly this class of error).
        let stray = Identifier {
            name: "Z".to_string(),
            id: 99,
        };
        let _ = Ty::Var(stray).is_codata(&[], &[]);
    }
}

#[cfg(test)]
mod check_tests {
    use super::{Identifier, Ty, TypeArgs};
    use crate::{
        syntax::TypingContext,
        typing::{check::Checked, env::GlobalEnv},
    };
    extern crate self as core_lang;
    use core_macros::{data, id, tparam, tvar, ty};

    #[test]
    fn check_fails_for_undeclared_type_var() {
        let t = tvar!(id!("A", 1));

        let res = t.check(&[], &TypingContext::default(), &GlobalEnv::default());
        assert!(res.is_err());
    }

    #[test]
    fn check_succeeds_for_declared_type_var() {
        let t = tvar!(id!("A", 1));

        let res = t.check(
            &[id!("A", 1)],
            &TypingContext::default(),
            &GlobalEnv::default(),
        );
        assert!(res.is_ok());
    }

    #[test]
    fn check_fails_for_undeclared_type_decl() {
        let ty_decl = Ty::Decl {
            name: Identifier::new("List".to_string()),
            type_args: TypeArgs { args: vec![] },
        };

        let res = ty_decl.check(&[], &TypingContext::default(), &GlobalEnv::default());
        assert!(res.is_err());
    }

    #[test]
    fn check_arity_mismatch_against_declaration() {
        // create a data declaration: List[A]
        let list = data!(id!("List"), [], [tparam!(id!("A", 1), "+")]);

        // arity mismatch: List[] against List[A]
        let ty_bad = ty!(id!("List"));

        let res = ty_bad.check(
            &[],
            &TypingContext::default(),
            &GlobalEnv::new(&[list], &[], &[]),
        );
        assert!(res.is_err());
    }

    #[test]
    fn check_succeeds_for_type_var_arg_with_type_params() {
        // create a data declaration: List[A]
        let list = data!(id!("List"), [], [tparam!(id!("A", 1), "+")]);

        // List[A] where A is a type variable declared in the current context
        let ty_var_arg = ty!(id!("List"), [tvar!(id!("A", 1))]);

        let res = ty_var_arg.check(
            &[id!("A", 1)],
            &TypingContext::default(),
            &GlobalEnv::new(&[list], &[], &[]),
        );
        assert!(res.is_ok());
    }
}

#[cfg(test)]
mod specialize_tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        mono::{
            erasure::ErasedDecls,
            naming_table::NamingTable,
            solver::Solution,
            specialize::{Specialize, SpecializeContext},
        },
        syntax::{Ty, types::TypeArgs},
    };
    extern crate self as core_lang;
    use core_macros::{data, id, tparam, tvar, ty};

    #[test]
    fn specialize_ground_i64_is_identity() {
        let solution = Solution::default();
        let table = NamingTable::build(&solution, &[], &[], &[], &ErasedDecls::default());
        let erased = ErasedDecls::default();
        let ctx = &SpecializeContext::ground(&table, &erased);

        let result = Ty::I64.specialize(ctx);
        assert_eq!(result, Ty::I64);
    }

    #[test]
    fn specialize_type_variable_via_subst() {
        // A -> i64 under an active substitution, as happens while
        // specializing the body of a polymorphic declaration.
        let solution = Solution::default();
        let table = NamingTable::build(&solution, &[], &[], &[], &ErasedDecls::default());
        let erased = ErasedDecls::default();
        let params = vec![id!("A", 1)];
        let args = vec![ty!("int")];
        let ctx = &SpecializeContext::with_subst(&table, &params, &args, &erased);

        let result = tvar!(id!("A", 1)).specialize(ctx);
        assert_eq!(result, ty!("int"));
    }

    #[test]
    fn specialize_decl_type_resolves_via_naming_table() {
        // List[i64] should resolve to the mangled name recorded in the
        // solution for List's node under instantiation [i64].
        let list_node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            list_node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));

        let list = data!(id!("List"), [], [tparam!(id!("A", 1), "+")]);
        let table = NamingTable::build(
            &solution,
            &[list.clone()],
            &[],
            &[],
            &ErasedDecls::default(),
        );
        let erased = ErasedDecls::default();
        let ctx = &SpecializeContext::ground(&table, &erased);

        let input = ty!(id!("List"), [ty!("int")]);
        let result = input.specialize(ctx);

        let expected_name = table.lookup(&list.name, &[ty!("int")]).clone();
        assert_eq!(
            result,
            Ty::Decl {
                name: expected_name,
                type_args: TypeArgs { args: vec![] }
            }
        );
    }

    #[test]
    #[should_panic(expected = "no specialized name recorded")]
    fn specialize_decl_type_panics_for_unsolved_instantiation() {
        let list_node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            list_node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));

        let list = data!(id!("List"), [], [tparam!(id!("A", 1), "+")]);
        let table = NamingTable::build(
            &solution,
            &[list.clone()],
            &[],
            &[],
            &ErasedDecls::default(),
        );
        let erased = ErasedDecls::default();
        let ctx = &SpecializeContext::ground(&table, &erased);

        let input = ty!(id!("List"), [ty!(id!("Bool"))]);
        let _ = input.specialize(ctx);
    }
}
