//! This module defines pattern and copattern matches in Core.

use printer::tokens::{CASE, NEW};
use printer::*;

use crate::mono::constraints::{ConstraintCollector, FlowConstraintSet};
use crate::mono::errors::Error;
use crate::syntax::declaration::{Polarity, TypeDeclaration, lookup_type_declaration};
use crate::syntax::types::TypeArgs;
use crate::traits::*;
use crate::typing::check::{Checked, instantiate_type_params};
use crate::typing::errors::{LocatedTypeError, TypeError};
use crate::{bail, syntax::*};

use std::collections::BTreeSet;

/// This struct defines pattern and copattern matches in Core. It consists of the information that
/// determines whether it is a match (if `C` is instantiated with [`Cns`]) or a comatch
/// (if `C` is instantiated with [`Prd`]), of a list of clauses, and of the type. The type
/// parameter `S` determines whether the bodies of the clauses unfocused ([`Statement`]) or focused
/// ([`FsStatement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCase<C: Chi, S = Statement> {
    /// Whether we have a match or comatch
    pub prdcns: C,
    /// The list of clauses
    pub clauses: Vec<Clause<C, S>>,
    /// The type
    pub ty: Ty,
}

#[allow(type_alias_bounds)]
pub type FsXCase<C: Chi> = XCase<C, FsStatement>;

impl<C: Chi, S> Typed for XCase<C, S> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<C: Chi, S: Print> Print for XCase<C, S> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let case = if self.prdcns.is_prd() {
            alloc.keyword(NEW)
        } else {
            alloc.keyword(CASE)
        };

        case.append(alloc.space())
            .append(super::clause::print_clauses(&self.clauses, cfg, alloc))
    }
}

impl<C: Chi> From<XCase<C>> for Term<C> {
    fn from(value: XCase<C>) -> Self {
        Term::XCase(value)
    }
}

impl<C: Chi> From<FsXCase<C>> for FsTerm<C> {
    fn from(value: FsXCase<C>) -> Self {
        FsTerm::XCase(value)
    }
}

impl<C: Chi> Subst for XCase<C> {
    type Target = XCase<C>;
    fn subst_sim(
        mut self,
        prod_subst: &[(Identifier, Term<Prd>)],
        cons_subst: &[(Identifier, Term<Cns>)],
    ) -> Self::Target {
        self.clauses = self.clauses.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl<C: Chi> TypedFreeVars for XCase<C> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.clauses.typed_free_vars(vars);
    }
}

impl<C: Chi> Uniquify for XCase<C> {
    fn uniquify(mut self, max_id: &mut ID) -> XCase<C> {
        self.clauses = self.clauses.uniquify(max_id);
        self
    }
}

impl<C: Chi> Focusing for XCase<C> {
    type Target = FsXCase<C>;
    // focus(cocase {cases}) = cocase { focus(cases) } AND focus(case {cases}) = case { focus(cases) }
    fn focus(self, max_id: &mut ID) -> Self::Target {
        XCase {
            prdcns: self.prdcns,
            clauses: self.clauses.focus(max_id),
            ty: self.ty,
        }
    }
}

impl Bind for XCase<Prd> {
    // bind(new { cases }[k] = ⟨ new { focus(cases) } | ~μx.k(x) ⟩
    fn bind(self, k: Continuation, max_id: &mut ID) -> FsStatement {
        let ty = self.ty.clone();
        let new_var = fresh_var(max_id);
        let new_binding = ContextBinding {
            var: new_var.clone(),
            chi: Chirality::Prd,
            ty: ty.clone(),
        };
        let cns = Mu::tilde_mu(new_var, k(new_binding, max_id), self.ty.clone());
        FsCut::new(self.focus(max_id), cns, ty).into()
    }
}
impl Bind for XCase<Cns> {
    // bind(case { cases }[k] = ⟨ μa.k(a) } | case { focus(cases) ⟩
    fn bind(self, k: Continuation, max_id: &mut ID) -> FsStatement {
        let ty = self.ty.clone();
        let new_covar = fresh_covar(max_id);
        let new_binding = ContextBinding {
            var: new_covar.clone(),
            chi: Chirality::Cns,
            ty: ty.clone(),
        };
        let prd = Mu::mu(new_covar, k(new_binding, max_id), self.ty.clone());
        FsCut::new(prd, self.focus(max_id), ty).into()
    }
}

impl<C: Chi> SubstVar for FsXCase<C> {
    type Target = FsXCase<C>;
    fn subst_sim(mut self, subst: &[(ID, Identifier)]) -> Self::Target {
        self.clauses = self.clauses.subst_sim(subst);
        self
    }
}

impl<C: Chi> TypedFreeVars for FsXCase<C> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.clauses.typed_free_vars(vars);
    }
}

impl<C: Chi> ConstraintCollector for XCase<C> {
    fn collect_constraints(
        &self,
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
    ) -> Result<FlowConstraintSet, Error> {
        let mut constraints = self
            .ty
            .collect_constraints(data_declarations, codata_declarations)?;
        constraints.extend(
            self.clauses
                .iter()
                .map(|clause| clause.collect_constraints(data_declarations, codata_declarations))
                .try_fold(FlowConstraintSet::new(), |mut acc, res| {
                    acc.extend(res?);
                    Ok(acc)
                })?,
        );
        Ok(constraints)
    }
}

impl<C: Chi> Checked for XCase<C> {
    fn check(
        &self,
        type_params: &[Identifier],
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
        defs: &[Def],
    ) -> Result<(), LocatedTypeError> {
        // check well-formedness of the type
        self.ty
            .check(type_params, data_declarations, codata_declarations, defs)?;

        // check that the type is a declaration type and get the declaration
        let (type_name, concrete_type_args) = match &self.ty {
            Ty::Decl { name, type_args } => Ok((name, &type_args.args)),
            _ => bail!(TypeError::TypeMismatch {
                expected: Ty::Decl {
                    name: Identifier::new("<decl-type>".to_string()),
                    type_args: TypeArgs { args: vec![] },
                },
                got: self.ty.clone(),
                msg: Some("case/new requires a declared algebraic type".to_string()),
            }),
        }?;

        // branch on whether we have a case or a new and check against the corresponding declaration
        if self.prdcns.is_prd() {
            let decl = lookup_type_declaration(type_name, codata_declarations);
            check_xcase_against_decl(
                self,
                decl,
                type_name,
                type_params,
                concrete_type_args,
                data_declarations,
                codata_declarations,
                defs,
            )
        } else {
            let decl = lookup_type_declaration(type_name, data_declarations);
            check_xcase_against_decl(
                self,
                decl,
                type_name,
                type_params,
                concrete_type_args,
                data_declarations,
                codata_declarations,
                defs,
            )
        }
    }
}

/// Checks that the given case or cocase is well-typed against the given type declaration. This includes checking that the type arguments match the type parameters of the declaration, that for each clause, the xtor exists in the declaration, and that the context of each clause matches the argument types of the corresponding xtor in the declaration.
fn check_xcase_against_decl<P: Polarity, C: Chi>(
    xcase: &XCase<C>,
    decl: &TypeDeclaration<P>,
    type_name: &Identifier,
    type_params: &[Identifier],
    concrete_type_args: &[Ty],
    data_declarations: &[DataDeclaration],
    codata_declarations: &[CodataDeclaration],
    defs: &[Def],
) -> Result<(), LocatedTypeError> {
    // check that the number of type arguments matches the number of type parameters
    if decl.type_params.len() != concrete_type_args.len() {
        bail!(TypeError::ArityMismatch {
            expected: decl.type_params.len(),
            got: concrete_type_args.len(),
        });
    }

    for clause in &xcase.clauses {
        // check well-formedness of the clause
        clause.check(type_params, data_declarations, codata_declarations, defs)?;

        // check that the xtor exists in the declaration and get its signature
        let Some(sig) = decl.xtors.iter().find(|xt| xt.name == clause.xtor) else {
            bail!(TypeError::UndeclaredXtor {
                type_name: type_name.name.clone(),
                xtor_name: clause.xtor.name.clone(),
            });
        };

        // check that the number of binders in the clause matches the number of arguments in the xtor signature
        if sig.args.bindings.len() != clause.context.bindings.len() {
            bail!(TypeError::ArityMismatch {
                expected: sig.args.bindings.len(),
                got: clause.context.bindings.len(),
            });
        }

        // check that the types of the binders in the clause match the types of the arguments in the xtor signature, after instantiating the type parameters with the concrete type arguments
        for (expected_binding, actual_binding) in
            sig.args.bindings.iter().zip(clause.context.bindings.iter())
        {
            if expected_binding.chi != actual_binding.chi {
                bail!(TypeError::Contextual {
                    msg: format!(
                        "Chirality mismatch in clause '{}' for binder '{}'",
                        clause.xtor.name, actual_binding.var.name
                    ),
                });
            }

            let expected_ty = instantiate_type_params(
                &expected_binding.ty,
                &decl.type_params,
                concrete_type_args,
            );

            if actual_binding.ty != expected_ty {
                bail!(TypeError::TypeMismatch {
                    expected: expected_ty,
                    got: actual_binding.ty.clone(),
                    msg: Some(format!(
                        "Binder type mismatch in clause '{}'",
                        clause.xtor.name
                    )),
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::syntax::*;
    use crate::test_common::example_subst;
    use crate::traits::*;
    extern crate self as core_lang;
    use crate::typing::check::Checked;
    use core_macros::{
        bind, case, clause, cns, cocase, covar, ctor_sig, cut, data, fs_clause, fs_cut, id, prd,
        ty, var,
    };

    #[test]
    fn focus_clause() {
        let result = clause!(
            Prd,
            id!("apply"),
            [bind!(id!("x"), prd!()), bind!(id!("a"), cns!())],
            cut!(var!(id!("x")), covar!(id!("a")))
        )
        .focus(&mut Default::default());
        let expected = fs_clause!(
            Prd,
            id!("apply"),
            [bind!(id!("x"), prd!()), bind!(id!("a"), cns!())],
            fs_cut!(var!(id!("x")), covar!(id!("a")))
        );
        assert_eq!(result, expected)
    }

    fn example_cocase() -> XCase<Prd> {
        cocase!(
            [
                clause!(
                    Prd,
                    id!("fst"),
                    [bind!(id!("x"), prd!()), bind!(id!("a"), cns!())],
                    cut!(var!(id!("x")), covar!(id!("a")))
                ),
                clause!(Prd, id!("snd"), [], cut!(var!(id!("x")), covar!(id!("a"))))
            ],
            ty!(id!("LPairIntInt"))
        )
        .into()
    }

    fn example_case() -> XCase<Cns> {
        case!(
            [
                clause!(Cns, id!("Nil"), [], cut!(var!(id!("x")), covar!(id!("a")))),
                clause!(
                    Cns,
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!()),
                        bind!(id!("xs"), prd!(), ty!(id!("ListInt"))),
                        bind!(id!("a"), cns!())
                    ],
                    cut!(var!(id!("x")), covar!(id!("a")))
                )
            ],
            ty!(id!("ListInt"))
        )
        .into()
    }

    #[test]
    fn subst_case() {
        let subst = example_subst();
        let result = example_case().subst_sim(&subst.0, &subst.1);
        let expected = case!(
            [
                clause!(Cns, id!("Nil"), [], cut!(var!(id!("y")), covar!(id!("b")))),
                clause!(
                    Cns,
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!()),
                        bind!(id!("xs"), prd!(), ty!(id!("ListInt"))),
                        bind!(id!("a"), cns!())
                    ],
                    cut!(var!(id!("x")), covar!(id!("a")))
                )
            ],
            ty!(id!("ListInt"))
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_cocase() {
        let subst = example_subst();
        let result = example_cocase().subst_sim(&subst.0, &subst.1);
        let expected = cocase!(
            [
                clause!(
                    Prd,
                    id!("fst"),
                    [bind!(id!("x"), prd!()), bind!(id!("a"), cns!())],
                    cut!(var!(id!("x")), covar!(id!("a")))
                ),
                clause!(Prd, id!("snd"), [], cut!(var!(id!("y")), covar!(id!("b"))))
            ],
            ty!(id!("LPairIntInt"))
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn check_against_declaration() {
        let list = data!(id!("List"), [ctor_sig!(id!("Nil"), [])], []);

        let good_case: XCase<Cns> = case!(
            [clause!(
                Cns,
                id!("Nil"),
                [],
                cut!(var!(id!("x")), covar!(id!("a")))
            )],
            ty!(id!("List"))
        )
        .into();
        assert!(good_case.check(&[], &[list.clone()], &[], &[]).is_ok());
    }

    #[test]
    fn check_undeclared_type() {
        let list = data!(id!("List"), [ctor_sig!(id!("Nil"), [])], []);

        let wrong: XCase<Cns> = case!(
            [clause!(
                Cns,
                id!("Nil"),
                [],
                cut!(var!(id!("x")), covar!(id!("a")))
            )],
            ty!(id!("NonExistent"))
        )
        .into();
        assert!(wrong.check(&[], &[list.clone()], &[], &[]).is_err());
    }
}
