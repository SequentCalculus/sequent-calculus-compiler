use super::{Cns, Prd, PrdCns, Term, XVar};
use crate::{
    syntax::{statement::Cut, Covar, Statement, Var},
    traits::{
        free_vars::{fresh_covar, fresh_var, FreeV},
        substitution::Subst,
        transform::{Bind, Continuation, NamingTransformation, TransformState},
    },
};
use std::{collections::HashSet, fmt, rc::Rc};

// Mu
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu<T: PrdCns> {
    pub prdcns: T,
    pub variable: Var,
    pub statement: Rc<Statement>,
}

impl<T: PrdCns> std::fmt::Display for Mu<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.prdcns.is_prd() {
            format!("mu '{}", self.variable)
        } else {
            format!("mutilde {}", self.variable)
        };
        write!(f, "{}. {}", prefix, self.statement)
    }
}

impl<T: PrdCns> FreeV for Mu<T> {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = FreeV::free_vars(Rc::as_ref(&self.statement));
        if self.prdcns.is_cns() {
            free_vars.remove(&self.variable);
        }
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.statement.free_covars();
        if self.prdcns.is_prd() {
            free_covars.remove(&self.variable);
        }
        free_covars
    }
}

impl<T: PrdCns> From<Mu<T>> for Term<T> {
    fn from(value: Mu<T>) -> Self {
        Term::Mu(value)
    }
}

impl Subst for Mu<Prd> {
    type Target = Mu<Prd>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Mu<Prd> {
        let Mu {
            prdcns: _,
            variable,
            statement,
        } = self;
        let mut free_covars: HashSet<Covar> = statement.free_covars();
        for (cons, covar) in cons_subst.iter() {
            free_covars.extend(cons.free_covars());
            free_covars.insert(covar.clone());
        }
        for (prod, _) in prod_subst.iter() {
            free_covars.extend(prod.free_covars());
        }
        let new_covar: Covar = fresh_covar(&free_covars);
        let new_statement: Rc<Statement> = statement.subst_covar(
            XVar {
                prdcns: Cns,
                var: new_covar.clone(),
            }
            .into(),
            variable.clone(),
        );
        Mu {
            prdcns: Prd,
            variable: new_covar,
            statement: new_statement.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Subst for Mu<Cns> {
    type Target = Mu<Cns>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Mu<Cns> {
        let Mu {
            prdcns: _,
            variable,
            statement,
        } = self;
        let mut free_vars: HashSet<Var> = statement.free_vars();
        for (prod, var) in prod_subst.iter() {
            free_vars.extend(prod.free_vars());
            free_vars.insert(var.clone());
        }
        for (cons, _) in cons_subst.iter() {
            free_vars.extend(cons.free_vars());
        }
        let new_var: Var = fresh_var(&free_vars);
        let new_statement: Rc<Statement> = statement.subst_var(
            XVar {
                prdcns: Prd,
                var: new_var.clone(),
            }
            .into(),
            variable.clone(),
        );
        Mu {
            prdcns: Cns,
            variable: new_var,
            statement: new_statement.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl<T: PrdCns> NamingTransformation for Mu<T> {
    type Target = Mu<T>;
    ///N(μa.s) = μa.N(s)
    fn transform(self, state: &mut TransformState) -> Self::Target {
        state.used_covars.insert(self.variable.clone());
        Mu {
            prdcns: self.prdcns,
            variable: self.variable,
            statement: self.statement.transform(state),
        }
    }
}

impl Bind for Mu<Prd> {
    ///bind(μa.s)[k] = ⟨μa.N(s) | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        state.used_covars.insert(self.variable.clone());
        let new_var = state.fresh_var();
        Cut {
            producer: Rc::new(Term::Mu(self.transform(state))),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: new_var.clone(),
                    statement: Rc::new(k(new_var, state)),
                }
                .into(),
            ),
        }
        .into()
    }
}

impl Bind for Mu<Cns> {
    /// bind(~μx.s)[k] = ⟨μa.k(a) | ~μx.N(s)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        state.used_vars.insert(self.variable.clone());
        let new_covar = state.fresh_covar();
        Cut {
            producer: Rc::new(Term::Mu(Mu {
                prdcns: Prd,
                variable: new_covar.clone(),
                statement: Rc::new(k(new_covar, state)),
            })),
            consumer: Rc::new(Term::Mu(self.transform(state))),
        }
        .into()
    }
}

#[cfg(test)]
mod transform_tests {
    use super::{Bind, NamingTransformation};

    use crate::syntax::{
        statement::Cut,
        term::{Cns, Literal, Mu, Prd, XVar},
        Statement,
    };
    use std::rc::Rc;

    fn example_mu1() -> Mu<Prd> {
        Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            statement: Rc::new(Statement::Done()),
        }
    }
    fn example_mu2() -> Mu<Prd> {
        Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn transform_mu1() {
        let result = example_mu1().transform(&mut Default::default());
        let expected = example_mu1();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_mu2() {
        let result = example_mu2().transform(&mut Default::default());
        let expected = example_mu2();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_mu1() {
        let result =
            example_mu1().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(example_mu1().into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn bind_mu2() {
        let result =
            example_mu2().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(example_mu2().into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod mu_tests {
    use super::{Cns, FreeV, Mu, Prd, Subst, Term};
    use crate::syntax::{statement::Cut, term::XVar, Covar, Var};
    use std::{collections::HashSet, rc::Rc};

    fn example_mu() -> Mu<Prd> {
        Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into()
    }

    fn example_mu_tilde() -> Mu<Cns> {
        Mu {
            prdcns: Cns,
            variable: "x".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into()
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
            }
            .into(),
            "x".to_owned(),
        )]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(
            XVar {
                prdcns: Cns,
                var: "b".to_owned(),
            }
            .into(),
            "a".to_owned(),
        )]
    }

    #[test]
    fn display_mu() {
        let result = format!("{}", example_mu());
        let expected = "mu 'a. <x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_mu_tilde() {
        let result = format!("{}", example_mu_tilde());
        let expected = "mutilde x. <x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_mu() {
        let result = example_mu().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_mu_tilde() {
        let result = example_mu_tilde().free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_mu() {
        let result = example_mu().free_covars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_mu_tilde() {
        let result = example_mu_tilde().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_mu() {
        let result = example_mu().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "y".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a0".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_mutilde() {
        let result = example_mu_tilde().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Mu {
            prdcns: Cns,
            variable: "x0".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x0".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "b".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }
}
