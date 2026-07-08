//! This module defines constructors and destructors in Core.

use printer::*;

use crate::mono::constraints::{ConstraintCollector, FlowConstraintSet};
use crate::mono::errors::MonoError;
use crate::mono::specialize::{Specialize, SpecializeContext};
use crate::traits::*;
use crate::typing::check::{Checked, check_arity};
use crate::typing::env::GlobalEnv;
use crate::typing::errors::{LocatedTypeError, TypeError};
use crate::{bail, syntax::*};

use core::panic;
use std::collections::BTreeSet;

/// This struct defines constructors and destructors in Core. It consists of the information that
/// determines whether it is a constructor (if `C` is instantiated with [`Prd`]) or a destructor
/// (if `C` is instantiated with [`Cns`]), a name for the xtor, the arguments of the xtor, and of
/// the type. The type parameter `A` determines whether this is the unfocused variant (if `A` is
/// instantiated with [`Arguments`], which is the default) or the focused variant (if `A` is
/// instantiated with [`TypingContext`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor<C: Chi, A = Arguments> {
    /// Whether we have a constructor or destructor
    pub prdcns: C,
    /// The xtor name
    pub name: Identifier,
    /// The arguments of the xtor
    pub args: A,
    /// The type of the xtor
    pub ty: Ty,
}

#[allow(type_alias_bounds)]
pub type FsXtor<C: Chi> = Xtor<C, TypingContext>;

impl<C: Chi, A> Typed for Xtor<C, A> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<C: Chi> Print for Xtor<C> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.entries.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        if self.prdcns.is_prd() {
            alloc
                .ctor(&self.name.print_to_string(Some(cfg)))
                .append(args.group())
        } else {
            alloc
                .dtor(&self.name.print_to_string(Some(cfg)))
                .append(args.group())
        }
    }
}

impl<C: Chi> Print for FsXtor<C> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.bindings.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        if self.prdcns.is_prd() {
            alloc
                .ctor(&self.name.print_to_string(Some(cfg)))
                .append(args)
        } else {
            alloc
                .dtor(&self.name.print_to_string(Some(cfg)))
                .append(args)
        }
    }
}

impl<C: Chi> From<Xtor<C>> for Term<C> {
    fn from(value: Xtor<C>) -> Self {
        Term::Xtor(value)
    }
}

impl<C: Chi> From<FsXtor<C>> for FsTerm<C> {
    fn from(value: FsXtor<C>) -> Self {
        FsTerm::Xtor(value)
    }
}

impl<C: Chi> Subst for Xtor<C> {
    type Target = Xtor<C>;
    fn subst_sim(
        mut self,
        prod_subst: &[(Identifier, Term<Prd>)],
        cons_subst: &[(Identifier, Term<Cns>)],
    ) -> Self::Target {
        self.args = self.args.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl<C: Chi> SubstVar for FsXtor<C> {
    type Target = FsXtor<C>;
    fn subst_sim(mut self, subst: &[(ID, Identifier)]) -> Self::Target {
        self.args = self.args.subst_sim(subst);
        self
    }
}

impl<C: Chi> TypedFreeVars for Xtor<C> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.args.typed_free_vars(vars);
    }
}

impl<C: Chi> TypedFreeVars for FsXtor<C> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.extend(self.args.bindings.iter().cloned());
    }
}

impl<C: Chi> Uniquify for Xtor<C> {
    fn uniquify(mut self, max_id: &mut ID) -> Xtor<C> {
        self.args = self.args.uniquify(max_id);
        self
    }
}

impl<C: Chi> Focusing for Xtor<C> {
    type Target = FsTerm<C>;
    fn focus(self, _: &mut ID) -> Self::Target {
        panic!("Constructors and destructors should always be focused in cuts directly");
    }
}

impl Bind for Xtor<Prd> {
    // bind(K(t_i))[k] = bind(t_i)[λas.⟨ K(as) | ~μx.k(x) ⟩]
    fn bind(self, k: Continuation, max_id: &mut ID) -> FsStatement {
        bind_many(
            self.args.into(),
            Box::new(|bindings, max_id: &mut ID| {
                let new_var = fresh_var(max_id);
                let new_binding = ContextBinding {
                    var: new_var.clone(),
                    chi: Chirality::Prd,
                    ty: self.ty.clone(),
                };
                FsCut::new(
                    FsTerm::Xtor(FsXtor {
                        prdcns: self.prdcns,
                        name: self.name,
                        args: bindings.into(),
                        ty: self.ty.clone(),
                    }),
                    Mu::tilde_mu(new_var, k(new_binding, max_id), self.ty.clone()),
                    self.ty,
                )
                .into()
            }),
            max_id,
        )
    }
}
impl Bind for Xtor<Cns> {
    // bind(D(t_i))[k] = bind(t_i)[λas.⟨ μa.k(a) | D(as) ⟩]
    fn bind(self, k: Continuation, max_id: &mut ID) -> FsStatement {
        bind_many(
            self.args.into(),
            Box::new(|bindings, max_id: &mut ID| {
                let new_covar = fresh_covar(max_id);
                let new_binding = ContextBinding {
                    var: new_covar.clone(),
                    chi: Chirality::Cns,
                    ty: self.ty.clone(),
                };
                FsCut::new(
                    Mu::mu(new_covar, k(new_binding, max_id), self.ty.clone()),
                    FsTerm::Xtor(FsXtor {
                        prdcns: self.prdcns,
                        name: self.name,
                        args: bindings.into(),
                        ty: self.ty.clone(),
                    }),
                    self.ty,
                )
                .into()
            }),
            max_id,
        )
    }
}

impl<C: Chi> ConstraintCollector for Xtor<C> {
    fn collect_constraints(&self, env: &GlobalEnv) -> Result<FlowConstraintSet, MonoError> {
        let mut constraints = self.ty.collect_constraints(env)?;
        constraints.extend(self.args.collect_constraints(env)?);
        Ok(constraints)
    }
}

impl<C: Chi> Specialize for Xtor<C> {
    fn specialize(&self, context: SpecializeContext) -> Self {
        Xtor {
            prdcns: self.prdcns.clone(),
            name: self.name.clone(),
            args: self.args.specialize(context),
            ty: self.ty.specialize(context),
        }
    }
}

impl<C: Chi> Checked for Xtor<C> {
    fn check(
        &self,
        type_params: &[Identifier],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        self.ty.check(type_params, context, env)?;
        self.args.check(type_params, context, env)?;

        let Ty::Decl { name, .. } = &self.ty else {
            bail!(TypeError::Contextual {
                msg: "Expected TypeDeclaration".to_string()
            })
        };

        // lookup the type declaration and check that the name of the xtor is defined on this declaration
        if self.prdcns.is_prd() {
            let data_decl = env.lookup_data_decl(name).ok_or_else(|| {
                LocatedTypeError::new(TypeError::UndeclaredType(name.name.clone()))
            })?;

            let Some(xtor) = data_decl.xtors.iter().find(|xtor| xtor.name == self.name) else {
                bail!(TypeError::UndeclaredXtor {
                    type_name: name.name.clone(),
                    xtor_name: self.name.name.clone()
                })
            };

            check_arity(xtor.args.bindings.len(), self.args.entries.len())?;
        } else {
            let codata_decl = env.lookup_codata_decl(name).ok_or_else(|| {
                LocatedTypeError::new(TypeError::UndeclaredType(name.name.clone()))
            })?;

            let Some(xtor) = codata_decl.xtors.iter().find(|xtor| xtor.name == self.name) else {
                bail!(TypeError::UndeclaredXtor {
                    type_name: name.name.clone(),
                    xtor_name: self.name.name.clone()
                })
            };

            check_arity(xtor.args.bindings.len(), self.args.entries.len())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod xtor_tests {
    use printer::Print;

    use super::Subst;

    use crate::syntax::*;
    use crate::test_common::example_subst;
    extern crate self as core_lang;
    use core_macros::{ctor, id, ty, var};

    fn example() -> Xtor<Prd> {
        ctor!(
            id!("Cons"),
            [var!(id!("x")), var!(id!("xs"), ty!(id!("ListInt")))],
            ty!(id!("ListInt"))
        )
    }

    #[test]
    fn display_const() {
        assert_eq!(example().print_to_string(None), "Cons(x, xs)")
    }

    #[test]
    fn subst_const() {
        let subst = example_subst();
        let result = example().subst_sim(&subst.0, &subst.1);
        let expected = ctor!(
            id!("Cons"),
            [var!(id!("y")), var!(id!("xs"), ty!(id!("ListInt")))],
            ty!(id!("ListInt"))
        );
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod constraint_tests {
    use crate::mono::constraints::{FlowConstraint, FlowConstraintSet};
    use crate::syntax::types::TypeArgs;
    use crate::typing::env::GlobalEnv;
    use crate::{mono::constraints::ConstraintCollector, syntax::*};
    use std::collections::HashSet;
    extern crate self as core_lang;
    use core_macros::{
        bind, cns, codata, ctor, ctor_sig, data, dtor, dtor_sig, id, lit, prd, tvar, ty,
    };

    fn example_list() -> DataDeclaration {
        data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), []),
                ctor_sig!(
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))]))
                    ]
                )
            ],
            [id!("A", 1)]
        )
    }

    #[test]
    fn collect_constraint_cons() {
        let list = example_list();

        let cons: Xtor<Prd> = ctor!(
            id!("Cons"),
            [
                lit!(1),
                ctor!(
                    id!("Cons"),
                    [
                        lit!(2),
                        ctor!(id!("Nil"), [], ty!(id!("List"), [ty!("int")]))
                    ],
                    ty!(id!("List"), [ty!("int")])
                )
            ],
            ty!(id!("List"), [ty!("int")])
        );

        let constraints = cons
            .collect_constraints(&GlobalEnv::new(&[list], &[], &[]))
            .unwrap();

        let expected = FlowConstraintSet {
            constraints: HashSet::from_iter(vec![FlowConstraint {
                from: vec![Ty::I64],
                to: vec![id!("A", 1)],
            }]),
        };

        assert_eq!(constraints, expected)
    }

    #[test]
    fn collect_constraint_cons_nested() {
        let list = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), []),
                ctor_sig!(
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))])),
                        bind!(
                            id!("xxs"),
                            prd!(),
                            ty!(id!("List"), [ty!(id!("List"), [tvar!(id!("A", 1))])])
                        )
                    ]
                )
            ],
            [id!("A", 1)]
        );

        let cons: Xtor<Prd> = ctor!(
            id!("Cons"),
            [
                lit!(1),
                ctor!(id!("Nil"), [], ty!(id!("List"), [ty!("int")])),
                ctor!(
                    id!("Nil"),
                    [],
                    ty!(id!("List"), [ty!(id!("List"), [ty!("int")])])
                )
            ],
            ty!(id!("List"), [ty!("int")])
        );

        let constraints = cons
            .collect_constraints(&GlobalEnv::new(&[list], &[], &[]))
            .unwrap();
        let expected = FlowConstraintSet {
            constraints: HashSet::from_iter(vec![
                FlowConstraint {
                    from: vec![Ty::I64],
                    to: vec![id!("A", 1)],
                },
                FlowConstraint {
                    from: vec![Ty::Decl {
                        name: Identifier {
                            name: "List".to_string(),
                            id: 0,
                        },
                        type_args: TypeArgs {
                            args: vec![Ty::I64],
                        },
                    }],
                    to: vec![id!("A", 1)],
                },
            ]),
        };

        assert_eq!(constraints, expected)
    }

    #[test]
    fn collect_constraint_nil() {
        let list = example_list();

        let nil: Xtor<Prd> = ctor!(id!("Nil"), [], ty!(id!("List"), [ty!("int")]));

        let constraints = nil
            .collect_constraints(&GlobalEnv::new(&[list], &[], &[]))
            .unwrap();

        assert_eq!(
            constraints,
            FlowConstraintSet {
                constraints: HashSet::from_iter(vec![FlowConstraint {
                    from: vec![Ty::I64],
                    to: vec![id!("A", 1)]
                }])
            }
        )
    }

    #[test]
    fn collect_constraint_dtor() {
        let list = codata!(
            id!("List"),
            [
                dtor_sig!(id!("Head"), [bind!(id!("h"), cns!(), tvar!(id!("A", 1)))]),
                dtor_sig!(
                    id!("Tail"),
                    [bind!(
                        id!("t"),
                        cns!(),
                        ty!(id!("List"), [tvar!(id!("A", 1))])
                    )]
                )
            ],
            [id!("A", 1)]
        );

        let dtor: Xtor<Cns> = dtor!(id!("Head"), [lit!(1)], ty!(id!("List"), [ty!("int")]));

        let constraints = dtor
            .collect_constraints(&GlobalEnv::new(&[], &[list], &[]))
            .unwrap();

        let expected = FlowConstraintSet {
            constraints: HashSet::from_iter(vec![FlowConstraint {
                from: vec![Ty::I64],
                to: vec![id!("A", 1)],
            }]),
        };

        assert_eq!(constraints, expected)
    }
}
