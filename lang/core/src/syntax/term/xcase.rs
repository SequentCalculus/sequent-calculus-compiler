use printer::{
    theme::ThemeExt,
    tokens::{CASE, COCASE},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{Cns, Prd, PrdCns, Term};
use crate::{
    syntax::{
        types::{Ty, Typed},
        Clause, Covar, Var,
    },
    traits::{
        focus::{Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
    },
};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCase<T: PrdCns> {
    pub prdcns: T,
    pub clauses: Vec<Clause>,
    pub ty: Ty,
}

impl<T: PrdCns> Typed for XCase<T> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<T: PrdCns> Print for XCase<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.prdcns.is_prd() {
            alloc.keyword(COCASE).append(alloc.space()).append(
                alloc
                    .space()
                    .append(self.clauses.print(cfg, alloc))
                    .append(alloc.space())
                    .braces_anno(),
            )
        } else {
            alloc.keyword(CASE).append(alloc.space()).append(
                alloc
                    .space()
                    .append(self.clauses.print(cfg, alloc))
                    .append(alloc.space())
                    .braces_anno(),
            )
        }
    }
}

impl<T: PrdCns> From<XCase<T>> for Term<T> {
    fn from(value: XCase<T>) -> Self {
        Term::XCase(value)
    }
}

impl<T: PrdCns> FreeV for XCase<T> {
    fn free_vars(&self) -> HashSet<Var> {
        self.clauses.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.clauses.free_covars()
    }
}

impl<T: PrdCns> Subst for XCase<T> {
    type Target = XCase<T>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        XCase {
            prdcns: self.prdcns.clone(),
            clauses: self.clauses.subst_sim(prod_subst, cons_subst),
            ty: self.ty.clone(),
        }
    }
}

impl<T: PrdCns> Focusing for XCase<T> {
    type Target = crate::syntax_var::term::XCase;

    ///N(case {cases}) = case { N(cases) } AND N(cocase {cases}) = case { N(cases) }
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        crate::syntax_var::term::XCase {
            clauses: self.clauses.focus(state),
        }
    }
}

impl<T: PrdCns> Bind for XCase<T> {
    ///bind(case {cases)[k] = ⟨μa.k(a) | case N{cases}⟩
    ///AND bind(cocase {cases)[k] = ⟨μa.k(a) | case N{cases}⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> crate::syntax_var::Statement {
        let new_covar = state.fresh_covar();
        let prod = crate::syntax_var::term::Mu::mu(&new_covar, k(new_covar.clone(), state));
        let ty = self.ty.clone();
        crate::syntax_var::statement::Cut::new(ty.focus(state), prod, self.focus(state)).into()
    }
}

#[cfg(test)]
mod tests {
    use printer::Print;

    use super::{Covar, FreeV, Subst, Term, Var, XCase};
    use crate::syntax::{
        context::ContextBinding,
        statement::Cut,
        term::{Cns, Prd, XVar},
        types::Ty,
        Clause,
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_cocase() -> XCase<Prd> {
        XCase {
            prdcns: Prd,
            clauses: vec![
                Clause {
                    xtor: "Fst".to_owned(),
                    context: vec![
                        ContextBinding::VarBinding {
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::CovarBinding {
                            covar: "a".to_owned(),
                            ty: Ty::Int(),
                        },
                    ],
                    rhs: Rc::new(
                        Cut::new(
                            XVar::var("x", Ty::Int()),
                            XVar::covar("a", Ty::Int()),
                            Ty::Int(),
                        )
                        .into(),
                    ),
                },
                Clause {
                    xtor: "Snd".to_owned(),
                    context: vec![],
                    rhs: Rc::new(
                        Cut::new(
                            XVar::var("x", Ty::Int()),
                            XVar::covar("a", Ty::Int()),
                            Ty::Int(),
                        )
                        .into(),
                    ),
                },
            ],
            ty: Ty::Decl("LPairIntInt".to_owned()),
        }
        .into()
    }

    fn example_case() -> XCase<Cns> {
        XCase {
            prdcns: Cns,
            clauses: vec![
                Clause {
                    xtor: "Nil".to_owned(),
                    context: vec![],
                    rhs: Rc::new(
                        Cut::new(
                            XVar::var("x", Ty::Int()),
                            XVar::covar("a", Ty::Int()),
                            Ty::Int(),
                        )
                        .into(),
                    ),
                },
                Clause {
                    xtor: "Cons".to_owned(),
                    context: vec![
                        ContextBinding::VarBinding {
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::VarBinding {
                            var: "xs".to_owned(),
                            ty: Ty::Decl("ListInt".to_owned()),
                        },
                        ContextBinding::CovarBinding {
                            covar: "a".to_owned(),
                            ty: Ty::Int(),
                        },
                    ],
                    rhs: Rc::new(
                        Cut::new(
                            XVar::var("x", Ty::Int()),
                            XVar::covar("a", Ty::Int()),
                            Ty::Int(),
                        )
                        .into(),
                    ),
                },
            ],
            ty: Ty::Decl("ListInt".to_owned()),
        }
        .into()
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(XVar::var("y", Ty::Int()).into(), "x".to_owned())]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(XVar::covar("b", Ty::Int()).into(), "a".to_owned())]
    }

    #[test]
    fn display_cocase() {
        let result = example_cocase().print_to_string(None);
        let expected =
            "cocase { Fst(x : Int, 'a :cns Int) => <x | 'a>, Snd() => <x | 'a> }".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_case() {
        let result = example_case().print_to_string(None);
        let expected =
            "case { Nil() => <x | 'a>, Cons(x : Int, xs : ListInt, 'a :cns Int) => <x | 'a> }"
                .to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_cocase() {
        let result = example_cocase().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_case() {
        let result = example_case().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_cocase() {
        let result = example_cocase().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_case() {
        let result = example_case().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_case() {
        let result = example_case().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XCase {
            prdcns: Cns,
            clauses: vec![
                Clause {
                    xtor: "Nil".to_owned(),
                    context: vec![],
                    rhs: Rc::new(
                        Cut::new(
                            XVar::var("y", Ty::Int()),
                            XVar::covar("b", Ty::Int()),
                            Ty::Int(),
                        )
                        .into(),
                    ),
                },
                Clause {
                    xtor: "Cons".to_owned(),
                    context: vec![
                        ContextBinding::VarBinding {
                            var: "x0".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::VarBinding {
                            var: "x1".to_owned(),
                            ty: Ty::Decl("ListInt".to_owned()),
                        },
                        ContextBinding::CovarBinding {
                            covar: "a0".to_owned(),
                            ty: Ty::Int(),
                        },
                    ],
                    rhs: Rc::new(
                        Cut::new(
                            XVar::var("x0", Ty::Int()),
                            XVar::covar("a0", Ty::Int()),
                            Ty::Int(),
                        )
                        .into(),
                    ),
                },
            ],
            ty: Ty::Decl("ListInt".to_owned()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_cocase() {
        let result = example_cocase().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XCase {
            prdcns: Prd,
            clauses: vec![
                Clause {
                    xtor: "Fst".to_owned(),
                    context: vec![
                        ContextBinding::VarBinding {
                            var: "x0".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::CovarBinding {
                            covar: "a0".to_owned(),
                            ty: Ty::Int(),
                        },
                    ],
                    rhs: Rc::new(
                        Cut::new(
                            XVar::var("x0", Ty::Int()),
                            XVar::covar("a0", Ty::Int()),
                            Ty::Int(),
                        )
                        .into(),
                    ),
                },
                Clause {
                    xtor: "Snd".to_owned(),
                    context: vec![],
                    rhs: Rc::new(
                        Cut::new(
                            XVar::var("y", Ty::Int()),
                            XVar::covar("b", Ty::Int()),
                            Ty::Int(),
                        )
                        .into(),
                    ),
                },
            ],
            ty: Ty::Decl("LPairIntInt".to_owned()),
        };
        assert_eq!(result, expected)
    }
}
