//! This module defines pattern and copattern matches in Core.

use printer::tokens::{CASE, NEW};
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};

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
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Var, Term<Cns>)],
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
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> XCase<C> {
        let seen_vars_clone = seen_vars.clone();
        let used_vars_clone = used_vars.clone();
        self.clauses = self
            .clauses
            .into_iter()
            .map(|clause| {
                let mut seen_vars_clause = seen_vars_clone.clone();
                let mut used_vars_clause = used_vars_clone.clone();
                let clause = clause.uniquify(&mut seen_vars_clause, &mut used_vars_clause);
                seen_vars.extend(seen_vars_clause);
                used_vars.extend(used_vars_clause);
                clause
            })
            .collect();

        self
    }
}

impl<C: Chi> Focusing for XCase<C> {
    type Target = FsXCase<C>;
    // focus(cocase {cases}) = cocase { focus(cases) } AND focus(case {cases}) = case { focus(cases) }
    fn focus(self, used_vars: &mut HashSet<Var>) -> Self::Target {
        XCase {
            prdcns: self.prdcns,
            clauses: self.clauses.focus(used_vars),
            ty: self.ty,
        }
    }
}

impl Bind for XCase<Prd> {
    // bind(new { cases }[k] = ⟨ new { focus(cases) } | ~μx.k(x) ⟩
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let ty = self.ty.clone();
        let new_var = fresh_var(used_vars);
        let new_binding = ContextBinding {
            var: new_var.clone(),
            chi: Chirality::Prd,
            ty: ty.clone(),
        };
        let cns = Mu::tilde_mu(new_var, k(new_binding, used_vars), self.ty.clone());
        FsCut::new(self.focus(used_vars), cns, ty).into()
    }
}
impl Bind for XCase<Cns> {
    // bind(case { cases }[k] = ⟨ μa.k(a) } | case { focus(cases) ⟩
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let ty = self.ty.clone();
        let new_covar = fresh_covar(used_vars);
        let new_binding = ContextBinding {
            var: new_covar.clone(),
            chi: Chirality::Cns,
            ty: ty.clone(),
        };
        let prd = Mu::mu(new_covar, k(new_binding, used_vars), self.ty.clone());
        FsCut::new(prd, self.focus(used_vars), ty).into()
    }
}

impl<C: Chi> SubstVar for FsXCase<C> {
    type Target = FsXCase<C>;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Self::Target {
        self.clauses = self.clauses.subst_sim(subst);
        self
    }
}

impl<C: Chi> TypedFreeVars for FsXCase<C> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.clauses.typed_free_vars(vars);
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::*;
    use crate::test_common::example_subst;
    use crate::traits::*;
    extern crate self as core_lang;
    use macros::{bind, case, clause, cns, cocase, covar, cut, fs_clause, fs_cut, prd, ty, var};

    #[test]
    fn focus_clause() {
        let result = clause!(
            Prd,
            "apply",
            [bind!("x", 0, prd!()), bind!("a", 0, cns!())],
            cut!(var!("x", 0), covar!("a", 0))
        )
        .focus(&mut Default::default());
        let expected = fs_clause!(
            Prd,
            "apply",
            [bind!("x", 0, prd!()), bind!("a", 0, cns!())],
            fs_cut!(var!("x", 0), covar!("a", 0))
        );
        assert_eq!(result, expected)
    }

    fn example_cocase() -> XCase<Prd> {
        cocase!(
            [
                clause!(
                    Prd,
                    "fst",
                    [bind!("x", 0, prd!()), bind!("a", 0, cns!())],
                    cut!(var!("x", 0), covar!("a", 0))
                ),
                clause!(Prd, "snd", [], cut!(var!("x", 0), covar!("a", 0)))
            ],
            ty!("LPairIntInt")
        )
        .into()
    }

    fn example_case() -> XCase<Cns> {
        case!(
            [
                clause!(Cns, "Nil", [], cut!(var!("x", 0), covar!("a", 0))),
                clause!(
                    Cns,
                    "Cons",
                    [
                        bind!("x", 0, prd!()),
                        bind!("xs", 0, prd!(), ty!("ListInt")),
                        bind!("a", 0, cns!())
                    ],
                    cut!(var!("x", 0), covar!("a", 0))
                )
            ],
            ty!("ListInt")
        )
        .into()
    }

    #[test]
    fn subst_case() {
        let subst = example_subst();
        let result = example_case().subst_sim(&subst.0, &subst.1);
        let expected = case!(
            [
                clause!(Cns, "Nil", [], cut!(var!("y", 0), covar!("b", 0))),
                clause!(
                    Cns,
                    "Cons",
                    [
                        bind!("x", 0, prd!()),
                        bind!("xs", 0, prd!(), ty!("ListInt")),
                        bind!("a", 0, cns!())
                    ],
                    cut!(var!("x", 0), covar!("a", 0))
                )
            ],
            ty!("ListInt")
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
                    "fst",
                    [bind!("x", 0, prd!()), bind!("a", 0, cns!())],
                    cut!(var!("x", 0), covar!("a", 0))
                ),
                clause!(Prd, "snd", [], cut!(var!("y", 0), covar!("b", 0)))
            ],
            ty!("LPairIntInt")
        );
        assert_eq!(result, expected)
    }
}
