use super::{Consumer, Covar, Var};
use crate::{
    syntax::{
        term::{Cns, Prd, Term, XVar},
        Statement,
    },
    traits::{
        free_vars::{fresh_var, FreeV},
        substitution::Subst,
    },
};
use std::{collections::HashSet, fmt, rc::Rc};

// MuTilde
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MuTilde {
    pub variable: Var,
    pub statement: Rc<Statement>,
}

impl std::fmt::Display for MuTilde {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mutilde {}. {}", self.variable, self.statement)
    }
}

impl FreeV for MuTilde {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.statement.free_vars();
        free_vars.remove(&self.variable);
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.statement.free_covars()
    }
}

impl From<MuTilde> for Consumer {
    fn from(value: MuTilde) -> Self {
        Consumer::MuTilde(value)
    }
}

impl Subst for MuTilde {
    type Target = MuTilde;

    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        let MuTilde {
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
        MuTilde {
            variable: new_var,
            statement: new_statement.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod mu_tilde_tests {
    use crate::{
        syntax::{
            statement::Cut,
            term::{Cns, Prd, Term, XVar},
            Covar, Covariable, MuTilde, Var, Variable,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_mu_tilde() -> MuTilde {
        MuTilde {
            variable: "x".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        Variable {
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        Covariable {
                            covar: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
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
    fn display_mu_tilde() {
        let result = format!("{}", example_mu_tilde());
        let expected = "mutilde x. <x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_mu_tilde() {
        let result = example_mu_tilde().free_vars();
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
    fn subst_mu_tilde() {
        let result = example_mu_tilde().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = MuTilde {
            variable: "x0".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        Variable {
                            var: "x0".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        Covariable {
                            covar: "b".to_owned(),
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
