//! This module defines constructors and destructors in Core.

use printer::*;

use crate::mono::constraints::{ConstraintCollector, FlowConstraintSet, collect_type_flow};
use crate::mono::erasure::erase_ty;
use crate::mono::errors::MonoError;
use crate::mono::specialize::{Specialize, SpecializeContext};
use crate::splitting::labeling::{
    DeclSignatures, FieldObservation, LabelAndUnify, SplitState, label_in,
};
use crate::splitting::rewrite::Rewrite;
use crate::splitting::split_table::SplitTable;
use crate::syntax::types::TypeArgs;
use crate::traits::*;
use crate::typing::check::{Checked, check_arity};
use crate::typing::env::GlobalEnv;
use crate::typing::errors::{LocatedTypeError, TypeError};
use crate::{bail, syntax::*};

use core::panic;
use std::collections::BTreeSet;

/// This struct defines constructors and destructors in Core. It consists of the information that
/// determines whether it is a constructor (if `C` is instantiated with [`Prd`]) or a destructor
/// (if `C` is instantiated with [`Cns`]), a name for the xtor, the type arguments of the xtor, the arguments of the xtor, and of
/// the type. The type parameter `A` determines whether this is the unfocused variant (if `A` is
/// instantiated with [`Arguments`], which is the default) or the focused variant (if `A` is
/// instantiated with [`TypingContext`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor<C: Chi, A = Arguments> {
    /// Whether we have a constructor or destructor
    pub prdcns: C,
    /// The xtor name
    pub name: Identifier,
    /// The type arguments of the xtor
    pub type_args: TypeArgs,
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
                .ctor(&self.name.name.print_to_string(Some(cfg)))
                .append(self.type_args.print_to_string(Some(cfg)))
                .append(args.group())
        } else {
            alloc
                .dtor(&self.name.name.print_to_string(Some(cfg)))
                .append(self.type_args.print_to_string(Some(cfg)))
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
                .append(self.type_args.print_to_string(Some(cfg)))
                .append(args)
        } else {
            alloc
                .dtor(&self.name.print_to_string(Some(cfg)))
                .append(self.type_args.print_to_string(Some(cfg)))
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
                        type_args: self.type_args,
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
                        type_args: self.type_args,
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
        let xtor_params: Vec<Identifier> = if self.prdcns.is_prd() {
            env.lookup_xtor_for_data_decl(&self.name)?
                .type_params
                .iter()
                .map(|p| p.id.clone())
                .collect()
        } else {
            env.lookup_xtor_for_codata_decl(&self.name)?
                .type_params
                .iter()
                .map(|p| p.id.clone())
                .collect()
        };

        constraints.extend(collect_type_flow(&self.type_args.args, &xtor_params)?);
        constraints.extend(self.args.collect_constraints(env)?);
        Ok(constraints)
    }
}

impl<C: Chi> Specialize for Xtor<C> {
    fn specialize(&self, context: &SpecializeContext) -> Self {
        let specialized_ty = self.ty.specialize(context);

        // If the surrounding declaration was erased, its own type arguments are no longer
        // reflected in `specialized_ty`, but they're still needed to pick the right
        // specialized xtor. Surface syntax never carries them explicitly at the call site
        // (they were always implicit via the expected type), so we recover them from `self.ty`.
        let extra_args: Vec<Ty> = match &self.ty {
            Ty::Decl { name, type_args } if context.erased_decls.is_erased(name) => type_args
                .args
                .iter()
                .map(|a| {
                    erase_ty(
                        &a.substitute((&context.subst.0, &context.subst.1)),
                        &context.erased_decls.0,
                    )
                })
                .collect(),
            _ => vec![],
        };

        let mut ground_type_args: Vec<Ty> = self
            .type_args
            .args
            .iter()
            .map(|a| {
                erase_ty(
                    &a.substitute((&context.subst.0, &context.subst.1)),
                    &context.erased_decls.0,
                )
            })
            .collect();
        ground_type_args.extend(extra_args);

        let mangled_name = if ground_type_args.is_empty() {
            self.name.clone()
        } else {
            context.table.lookup(&self.name, &ground_type_args).clone()
        };

        Xtor {
            prdcns: self.prdcns.clone(),
            name: mangled_name,
            type_args: TypeArgs::default(),
            args: self.args.specialize(context),
            ty: specialized_ty,
        }
    }
}

impl<C: Chi> Checked for Xtor<C> {
    fn check(
        &self,
        type_params: &[TypeParam],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        self.ty.check(type_params, context, env)?;
        self.type_args
            .args
            .iter()
            .try_for_each(|arg| arg.check(type_params, context, env))?;
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

impl<C: Chi> LabelAndUnify for Xtor<C> {
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self {
        // Label this occurrence's own explicit type arguments (instantiating the xtor's own
        // existential/universal type parameters, e.g. `Pack[List[int]](...)`) before using them.
        let type_args = TypeArgs {
            args: self
                .type_args
                .args
                .iter()
                .map(|a| state.label_ty(a))
                .collect(),
        };
        // `ty` is the concrete type of the whole xtor value (e.g. `Fun[i64, Fun[i64, i64]]` for a
        // destructor consumption). `label_in` below relies on it being a declared type.
        let ty = state.label_ty(&self.ty);
        if !matches!(ty, Ty::Decl { .. }) {
            panic!("Expected declaration type in Xtor to label, got {ty:?}");
        }
        let args = self.args.label_and_unify(state, sigs, scope);

        if !sigs.contains_key(&self.name) {
            panic!("missing signature for xtor: {}", self.name.name);
        }
        let owner = label_in(&ty).clone();
        // Every field's actual type is recorded per-occurrence via `FieldObservation` rather than
        // unified immediately: whether it should end up sharing a physical copy with some other
        // occurrence's field depends on whether their owners turn out equivalent, which is only
        // known once the whole walk finishes (see `merge_field_observations`).
        for (i, arg) in args.entries.iter().enumerate() {
            state.field_observations.push(FieldObservation {
                owner: owner.clone(),
                xtor: self.name.clone(),
                field_index: i,
                ty: arg.get_type(),
            });
        }

        Xtor {
            prdcns: self.prdcns.clone(),
            name: self.name.clone(),
            type_args,
            args,
            ty,
        }
    }
}

impl<C: Chi> Rewrite for Xtor<C> {
    fn rewrite(&self, table: &SplitTable) -> Self {
        let owner_label = label_in(&self.ty);
        Xtor {
            prdcns: self.prdcns.clone(),
            name: table.resolve_xtor_name(&self.name, owner_label).clone(),
            type_args: self.type_args.rewrite(table),
            args: self.args.rewrite(table),
            ty: self.ty.rewrite(table),
        }
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
        return ctor!(
            id!("Cons"),
            [],
            [var!(id!("x")), var!(id!("xs"), ty!(id!("ListInt")))],
            ty!(id!("ListInt"))
        );
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
            [],
            [var!(id!("y")), var!(id!("xs"), ty!(id!("ListInt")))],
            ty!(id!("ListInt"))
        );
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod label_and_unify_tests {
    use crate::splitting::labeling::{DeclSignature, DeclSignatures, LabelAndUnify, SplitState};
    use crate::syntax::*;
    use crate::traits::*;
    extern crate self as core_lang;
    use core_macros::{ctor, id, ty};

    #[test]
    fn label_and_unify_defers_the_field_to_an_observation_instead_of_unifying_eagerly() {
        let mut state = SplitState::default();

        let mut sigs = DeclSignatures::new();
        sigs.insert(
            id!("Wrap"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![],
                tys: vec![ty!(id!("Box"))],
            },
        );
        sigs.insert(
            id!("Pack"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![],
                tys: vec![],
            },
        );

        let example = ctor!(
            id!("Wrap"),
            [],
            [ctor!(id!("Pack"), [], [], ty!(id!("Box")))],
            ty!(id!("Wrapper"))
        );

        let result: Xtor<Prd> =
            example.label_and_unify(&mut state, &sigs, &TypingContext::default());
        let arg_ty = result.args.entries[0].get_type();

        // no signature-level anchor is ever minted or unified against -- the field's type is
        // recorded as an observation, owned by this occurrence's own (freshly labeled) `.ty`
        assert_eq!(state.field_observations.len(), 1);
        let owner = match &result.ty {
            Ty::Decl { name, .. } => name,
            _ => panic!("expected Ty::Decl"),
        };
        assert_eq!(&state.field_observations[0].owner, owner);
        assert_eq!(state.field_observations[0].ty, arg_ty);
        // the xtor's own type is freshly labeled, independent of the field-level observation
        assert!(matches!(result.ty, Ty::Decl { .. }));
    }

    #[test]
    fn label_and_unify_keeps_two_independent_occurrences_field_observations_separate() {
        let mut state = SplitState::default();
        let mut sigs = DeclSignatures::new();
        sigs.insert(
            id!("MkBar"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![],
                tys: vec![ty!(id!("Foo"))],
            },
        );
        sigs.insert(
            id!("MkFoo"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![],
                tys: vec![],
            },
        );

        let example = ctor!(
            id!("MkBar"),
            [],
            [ctor!(id!("MkFoo"), [], [], ty!(id!("Foo")))],
            ty!(id!("Bar"))
        );

        let first: Xtor<Prd> =
            example.label_and_unify(&mut state, &sigs, &TypingContext::default());
        let second: Xtor<Prd> =
            example.label_and_unify(&mut state, &sigs, &TypingContext::default());

        let arg_name = |x: &Xtor<Prd>| match x.args.entries[0].get_type() {
            Ty::Decl { name, .. } => name,
            _ => panic!("expected Ty::Decl"),
        };
        // nothing unifies the two independent `MkFoo` occurrences with each other
        assert_ne!(
            state.uf.find(&arg_name(&first)),
            state.uf.find(&arg_name(&second))
        );
        assert_eq!(state.field_observations.len(), 2);
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
        bind, cns, codata, ctor, ctor_sig, data, dtor, dtor_sig, id, lit, prd, tparam, tvar, ty,
    };

    fn example_list() -> DataDeclaration {
        return data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))]))
                    ]
                )
            ],
            [tparam!(id!("A", 1), "+")]
        );
    }

    fn box_decl() -> DataDeclaration {
        return data!(
            id!("Box"),
            [ctor_sig!(
                id!("Pack"),
                [tparam!(id!("A", 1), "+")],
                [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))]
            )],
            []
        );
    }

    fn runner_decl() -> CodataDeclaration {
        return codata!(
            id!("Runner"),
            [dtor_sig!(
                id!("Run"),
                [tparam!(id!("A", 1), "+")],
                [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))]
            )],
            []
        );
    }

    fn container_decl() -> CodataDeclaration {
        return codata!(
            id!("Container"),
            [dtor_sig!(
                id!("Wrap"),
                [tparam!(id!("S", 2), "+")],
                [bind!(id!("x"), prd!(), tvar!(id!("S", 2)))]
            )],
            [tparam!(id!("T", 1), "+")]
        );
    }

    #[test]
    fn collect_constraint_cons() {
        let list = example_list();

        let cons: Xtor<Prd> = ctor!(
            id!("Cons"),
            [],
            [
                lit!(1),
                ctor!(
                    id!("Cons"),
                    [],
                    [
                        lit!(2),
                        ctor!(id!("Nil"), [], [], ty!(id!("List"), [ty!("int")]))
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
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
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
            [tparam!(id!("A", 1), "+")]
        );

        let cons: Xtor<Prd> = ctor!(
            id!("Cons"),
            [],
            [
                lit!(1),
                ctor!(id!("Nil"), [], [], ty!(id!("List"), [ty!("int")])),
                ctor!(
                    id!("Nil"),
                    [],
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

        let nil: Xtor<Prd> = ctor!(id!("Nil"), [], [], ty!(id!("List"), [ty!("int")]));

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
                dtor_sig!(
                    id!("Head"),
                    [],
                    [bind!(id!("h"), cns!(), tvar!(id!("A", 1)))]
                ),
                dtor_sig!(
                    id!("Tail"),
                    [],
                    [bind!(
                        id!("t"),
                        cns!(),
                        ty!(id!("List"), [tvar!(id!("A", 1))])
                    )]
                )
            ],
            [tparam!(id!("A", 1), "+")]
        );

        let dtor: Xtor<Cns> = dtor!(id!("Head"), [], [lit!(1)], ty!(id!("List"), [ty!("int")]));

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

    #[test]
    fn collect_constraint_existential_ctor() {
        let box_decl = box_decl();

        let pack: Xtor<Prd> = ctor!(
            id!("Pack"),
            [ty!(id!("List"), [ty!("int")])],
            [ctor!(id!("Nil"), [], [], ty!(id!("List"), [ty!("int")]))],
            ty!(id!("Box"))
        );

        let constraints = pack
            .collect_constraints(&GlobalEnv::new(&[box_decl, example_list()], &[], &[]))
            .unwrap();

        let expected = FlowConstraintSet {
            constraints: HashSet::from_iter(vec![
                FlowConstraint {
                    from: vec![Ty::Decl {
                        name: id!("List"),
                        type_args: TypeArgs {
                            args: vec![Ty::I64],
                        },
                    }],
                    to: vec![id!("A", 1)],
                },
                FlowConstraint {
                    from: vec![Ty::I64],
                    to: vec![id!("A", 1)],
                },
            ]),
        };

        assert_eq!(constraints, expected)
    }

    #[test]
    fn collect_constraint_universal_dtor() {
        let runner = runner_decl();

        let run: Xtor<Cns> = dtor!(id!("Run"), [ty!("int")], [lit!(1)], ty!(id!("Runner")));

        let constraints = run
            .collect_constraints(&GlobalEnv::new(&[], &[runner], &[]))
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
    fn collect_constraint_dtor_with_decl_and_own_type_params() {
        let container = container_decl();

        let wrap: Xtor<Cns> = dtor!(
            id!("Wrap"),
            [ty!("int")],
            [lit!(1)],
            ty!(id!("Container"), [ty!(id!("List"), [ty!("int")])])
        );

        let constraints = wrap
            .collect_constraints(&GlobalEnv::new(&[], &[container], &[]))
            .unwrap();

        let expected = FlowConstraintSet {
            constraints: HashSet::from_iter(vec![
                FlowConstraint {
                    from: vec![Ty::Decl {
                        name: id!("List"),
                        type_args: TypeArgs {
                            args: vec![Ty::I64],
                        },
                    }],
                    to: vec![id!("T", 1)],
                },
                FlowConstraint {
                    from: vec![Ty::I64],
                    to: vec![id!("S", 2)],
                },
            ]),
        };

        assert_eq!(constraints, expected)
    }
}
