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
        syntax::{
            statement::Cut,
            term::{Cns, Mu, Prd, Term, XVar},
            types::Ty,
            Statement,
        },
        traits::free_vars::FreeV,
    };
    use std::{collections::HashSet, rc::Rc};
    #[test]
    fn free_vars_vec() {
        let terms: Vec<Term<Prd>> = vec![
            XVar {
                prdcns: Prd,
                var: "x".to_owned(),
                ty: Ty::Int(),
            }
            .into(),
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
                ty: Ty::Int(),
            }
            .into(),
            Mu {
                prdcns: Prd,
                variable: "a".to_owned(),
                var_ty: Ty::Int(),
                statement: Rc::new(
                    Cut {
                        producer: Rc::new(
                            XVar {
                                prdcns: Prd,
                                var: "z".to_owned(),
                                ty: Ty::Int(),
                            }
                            .into(),
                        ),
                        ty: Ty::Int(),
                        consumer: Rc::new(
                            Mu {
                                prdcns: Cns,
                                variable: "x".to_owned(),
                                var_ty: Ty::Int(),
                                statement: Rc::new(Statement::Done(Ty::Int())),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            }
            .into(),
        ];
        let result = terms.free_vars();
        let expected = HashSet::from(["x".to_owned(), "y".to_owned(), "z".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_vec() {
        let terms: Vec<Term<Cns>> = vec![
            XVar {
                prdcns: Cns,
                var: "a".to_owned(),
                ty: Ty::Int(),
            }
            .into(),
            XVar {
                prdcns: Cns,
                var: "b".to_owned(),
                ty: Ty::Int(),
            }
            .into(),
            Mu {
                prdcns: Cns,
                variable: "x".to_owned(),
                var_ty: Ty::Int(),
                statement: Rc::new(
                    Cut {
                        producer: Rc::new(
                            Mu {
                                prdcns: Prd,
                                variable: "a".to_owned(),
                                var_ty: Ty::Int(),
                                statement: Rc::new(Statement::Done(Ty::Int())),
                            }
                            .into(),
                        ),
                        ty: Ty::Int(),
                        consumer: Rc::new(
                            XVar {
                                prdcns: Cns,
                                var: "c".to_owned(),
                                ty: Ty::Int(),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            }
            .into(),
        ];
        let result = terms.free_covars();
        let expected = HashSet::from(["a".to_owned(), "b".to_owned(), "c".to_owned()]);
        assert_eq!(result, expected)
    }
}
