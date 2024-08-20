use std::collections::HashSet;

use crate::syntax::{Covar, Var};

/// Computing the free variables and covariables of a term.
pub trait FreeV {
    /// Compute the free variables of a term.
    fn free_vars(&self) -> HashSet<Var>;
    /// Compute the free covariables of a term.
    fn free_covars(&self) -> HashSet<Covar>;
}

impl<T: FreeV> FreeV for Vec<T> {
    fn free_vars(self: &Vec<T>) -> HashSet<Var> {
        self.iter().fold(HashSet::new(), |mut free_vars, t| {
            free_vars.extend(t.free_vars());
            free_vars
        })
    }
    fn free_covars(self: &Vec<T>) -> HashSet<Covar> {
        self.iter().fold(HashSet::new(), |mut free_covars, t| {
            free_covars.extend(t.free_covars());
            free_covars
        })
    }
}

pub fn fresh_var(xs: &HashSet<Var>) -> Var {
    fresh_var_n(xs, 0)
}

fn fresh_var_n(xs: &HashSet<Var>, mut n: i32) -> Var {
    let mut new_var: Var = format!("x{}", n);
    while xs.contains(&new_var) {
        n += 1;
        new_var = format!("x{}", n);
    }
    new_var
}

pub fn fresh_covar(xs: &HashSet<Covar>) -> Covar {
    fresh_covar_n(xs, 0)
}

fn fresh_covar_n(xs: &HashSet<Covar>, mut n: i32) -> Covar {
    let mut new_covar: Covar = format!("a{}", n);
    while xs.contains(&new_covar) {
        n += 1;
        new_covar = format!("a{}", n);
    }
    new_covar
}

#[cfg(test)]
mod free_v_tests {
    use crate::{
        syntax::{Consumer, Covariable, Cut, Mu, MuTilde, Producer, Statement, Variable},
        traits::free_vars::FreeV,
    };
    use std::{collections::HashSet, rc::Rc};
    #[test]
    fn free_vars_vec() {
        let result = vec![
            <Variable as Into<Producer>>::into(Variable {
                var: "x".to_owned(),
            }),
            Variable {
                var: "y".to_owned(),
            }
            .into(),
            Mu {
                covariable: "a".to_owned(),
                statement: Rc::new(
                    Cut {
                        producer: Rc::new(
                            Variable {
                                var: "z".to_owned(),
                            }
                            .into(),
                        ),
                        consumer: Rc::new(
                            MuTilde {
                                variable: "x".to_owned(),
                                statement: Rc::new(Statement::Done()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            }
            .into(),
        ]
        .free_vars();
        let expected = HashSet::from(["x".to_owned(), "y".to_owned(), "z".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_vec() {
        let result = vec![
            <Covariable as Into<Consumer>>::into(Covariable {
                covar: "a".to_owned(),
            }),
            Covariable {
                covar: "b".to_owned(),
            }
            .into(),
            MuTilde {
                variable: "x".to_owned(),
                statement: Rc::new(
                    Cut {
                        producer: Rc::new(
                            Mu {
                                covariable: "a".to_owned(),
                                statement: Rc::new(Statement::Done()),
                            }
                            .into(),
                        ),
                        consumer: Rc::new(
                            Covariable {
                                covar: "c".to_owned(),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            }
            .into(),
        ]
        .free_covars();
        let expected = HashSet::from(["a".to_owned(), "b".to_owned(), "c".to_owned()]);
        assert_eq!(result, expected)
    }
}
