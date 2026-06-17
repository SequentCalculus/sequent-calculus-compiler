//! This module defines types in Core.

use printer::tokens::I64;
use printer::*;

use crate::mono::constraints::{ConstraintCollector, FlowConstraintSet, collect_type_flow};
use crate::mono::errors::MonoError;
use crate::syntax::declaration::lookup_type_declaration;
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
    pub fn is_codata(&self, codata_types: &[CodataDeclaration]) -> bool {
        match self {
            Ty::I64 => false,
            Ty::Decl { name, .. } => codata_types
                .iter()
                .any(|declaration| declaration.name == *name),
            Ty::Var(_) => false,
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
                    bail!(TypeError::UndeclaredType(param.name.clone()))
                }
            }
            Ty::Decl { name, type_args } => {
                // check that the type name is declared as a data or codata type and get the type params
                let Some(declaration_type_params) = env.lookup_type_params(name) else {
                    bail!(TypeError::UndeclaredType(name.name.clone()))
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
    fn collect_constraints(
        &self,
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
    ) -> Result<FlowConstraintSet, MonoError> {
        match self {
            Ty::I64 => Ok(FlowConstraintSet::new()),
            Ty::Var(_) => Ok(FlowConstraintSet::new()),
            Ty::Decl { name, type_args } => {
                if self.is_codata(codata_declarations) {
                    let template = lookup_type_declaration(name, codata_declarations);
                    type_args.args.iter().zip(&template.type_params).try_fold(
                        FlowConstraintSet::new(),
                        |mut acc, (arg, param)| {
                            acc.extend(collect_type_flow(arg, &Ty::Var(param.clone()))?);
                            Ok(acc)
                        },
                    )
                } else {
                    let template = lookup_type_declaration(name, data_declarations);
                    type_args.args.iter().zip(&template.type_params).try_fold(
                        FlowConstraintSet::new(),
                        |mut acc, (arg, param)| {
                            acc.extend(collect_type_flow(arg, &Ty::Var(param.clone()))?);
                            Ok(acc)
                        },
                    )
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct TypeArgs {
    /// The type arguments
    pub args: Vec<Ty>,
}

impl Print for TypeArgs {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.args.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.args.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
                .brackets()
                .group()
        }
    }
}

#[cfg(test)]
mod type_tests {
    use super::{Identifier, Ty, TypeArgs};
    use crate::{
        syntax::TypingContext,
        typing::{check::Checked, env::GlobalEnv},
    };
    use printer::Print;
    extern crate self as core_lang;
    use core_macros::{data, id, tvar, ty};

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
        let list = data!(id!("List"), [], [id!("A", 1)]);

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
        let list = data!(id!("List"), [], [id!("A", 1)]);

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
