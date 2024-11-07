use printer::{
    tokens::{CASE, COCASE},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{Cns, Mu, Prd, PrdCns, Term};
use crate::{
    syntax::{statement::Cut, stringify_and_join, Clause, Covar, Statement, Var},
    traits::{
        focus::{Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
    },
};
use std::{collections::HashSet, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCase<T: PrdCns> {
    pub prdcns: T,
    pub clauses: Vec<Clause>,
}
impl<T: PrdCns> std::fmt::Display for XCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses_joined: String = stringify_and_join(&self.clauses);
        let prefix = if self.prdcns.is_prd() {
            "cocase"
        } else {
            "case"
        };
        write!(f, "{} {{ {} }}", prefix, clauses_joined)
    }
}

impl<T: PrdCns> Print for XCase<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.prdcns.is_prd() {
            alloc
                .text(COCASE)
                .append(self.clauses.print(cfg, alloc).braces_anno())
        } else {
            alloc
                .text(CASE)
                .append(self.clauses.print(cfg, alloc).braces_anno())
        }
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

impl<T: PrdCns> From<XCase<T>> for Term<T> {
    fn from(value: XCase<T>) -> Self {
        Term::XCase(value)
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
        }
    }
}

impl Focusing for XCase<Cns> {
    type Target = XCase<Cns>;

    ///N(case {cases}) = case { N(cases) }
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        XCase {
            prdcns: Cns,
            clauses: self.clauses.focus(state),
        }
    }
}

impl Focusing for XCase<Prd> {
    type Target = XCase<Prd>;
    ///N(cocase {cocases}) = cocase { N(cocases) }
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        XCase {
            prdcns: Prd,
            clauses: self.clauses.focus(state),
        }
    }
}

impl Bind for XCase<Cns> {
    ///bind(case {cases)[k] =  ⟨μa.k(a) | case N{cases}⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement {
        let new_covar = state.fresh_covar();
        Cut {
            consumer: Rc::new(Term::XCase(XCase {
                prdcns: Cns,
                clauses: self.clauses.focus(state),
            })),
            producer: Rc::new(Term::Mu(Mu {
                prdcns: Prd,
                variable: new_covar.clone(),
                statement: Rc::new(k(new_covar, state)),
            })),
        }
        .into()
    }
}

impl Bind for XCase<Prd> {
    ///bind(cocase {cocases)[k] = ⟨cocase N(cocases) | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement {
        let new_var = state.fresh_var();
        Cut {
            producer: Rc::new(Term::XCase(self.focus(state))),
            consumer: Rc::new(Term::Mu(Mu {
                prdcns: Cns,
                variable: new_var.clone(),
                statement: Rc::new(k(new_var, state)),
            })),
        }
        .into()
    }
}

#[cfg(test)]
mod xcase_tests {
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
                    rhs: Rc::new(Cut::new(XVar::var("x"), XVar::covar("a")).into()),
                },
                Clause {
                    xtor: "Snd".to_owned(),
                    context: vec![],
                    rhs: Rc::new(Cut::new(XVar::var("x"), XVar::covar("a")).into()),
                },
            ],
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
                    rhs: Rc::new(Cut::new(XVar::var("x"), XVar::covar("a")).into()),
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
                    rhs: Rc::new(Cut::new(XVar::var("x"), XVar::covar("a")).into()),
                },
            ],
        }
        .into()
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(XVar::var("y").into(), "x".to_owned())]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(XVar::covar("b").into(), "a".to_owned())]
    }

    #[test]
    fn display_cocase() {
        let result = format!("{}", example_cocase());
        let expected =
            "cocase { Fst(x : Int, 'a :cnt Int) => <x | 'a>, Snd() => <x | 'a> }".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_case() {
        let result = format!("{}", example_case());
        let expected =
            "case { Nil() => <x | 'a>, Cons(x : Int, xs : ListInt, 'a :cnt Int) => <x | 'a> }"
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
                    rhs: Rc::new(Cut::new(XVar::var("y"), XVar::covar("b")).into()),
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
                    rhs: Rc::new(Cut::new(XVar::var("x0"), XVar::covar("a0")).into()),
                },
            ],
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
                    rhs: Rc::new(Cut::new(XVar::var("x0"), XVar::covar("a0")).into()),
                },
                Clause {
                    xtor: "Snd".to_owned(),
                    context: vec![],
                    rhs: Rc::new(Cut::new(XVar::var("y"), XVar::covar("b")).into()),
                },
            ],
        };
        assert_eq!(result, expected)
    }
}
