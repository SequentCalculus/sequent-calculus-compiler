use std::rc::Rc;

use crate::syntax::{
    term::{Cns, Prd, Term},
    Covar, Var,
};

pub trait Subst: Clone {
    type Target: Clone;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target;

    fn subst_var(&self, prod: Term<Prd>, var: Var) -> Self::Target {
        self.subst_sim(&[(prod, var)], &[])
    }
    fn subst_covar(&self, cons: Term<Cns>, covar: Covar) -> Self::Target {
        self.subst_sim(&[], &[(cons, covar)])
    }
}

impl<T: Subst> Subst for Rc<T> {
    type Target = Rc<T::Target>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        Rc::new((**self).subst_sim(prod_subst, cons_subst))
    }
}

impl<T: Subst + Clone> Subst for Vec<T> {
    type Target = Vec<T::Target>;
    fn subst_sim(
        self: &Vec<T>,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Vec<T::Target> {
        self.iter()
            .map(|x| x.subst_sim(prod_subst, cons_subst))
            .collect()
    }
}

#[cfg(test)]
mod subst_tests {
    use crate::{
        syntax::term::{Cns, Prd, Term, XVar},
        traits::substitution::Subst,
    };
    use std::rc::Rc;

    #[test]
    fn subst_variable1() {
        let result = XVar {
            prdcns: Prd,
            var: "x".to_owned(),
        }
        .subst_var(
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
            }
            .into(),
            "x".to_owned(),
        );
        let expected = XVar {
            prdcns: Prd,
            var: "y".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_variable2() {
        let result = XVar {
            prdcns: Prd,
            var: "z".to_owned(),
        }
        .subst_var(
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
            }
            .into(),
            "x".to_owned(),
        );
        let expected = XVar {
            prdcns: Prd,
            var: "z".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_covariable1() {
        let result = XVar {
            prdcns: Cns,
            var: "a".to_owned(),
        }
        .subst_covar(
            XVar {
                prdcns: Cns,
                var: "b".to_owned(),
            }
            .into(),
            "a".to_owned(),
        );
        let expected = XVar {
            prdcns: Cns,
            var: "b".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_covariable2() {
        let result = XVar {
            prdcns: Cns,
            var: "c".to_owned(),
        }
        .subst_covar(
            XVar {
                prdcns: Cns,
                var: "b".to_owned(),
            }
            .into(),
            "a".to_owned(),
        );
        let expected = XVar {
            prdcns: Cns,
            var: "c".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_rc1() {
        let prod_subst = vec![(
            XVar {
                prdcns: Prd,
                var: "x".to_owned(),
            }
            .into(),
            "y".to_owned(),
        )];
        let cons_subst = vec![(
            XVar {
                prdcns: Cns,
                var: "a".to_owned(),
            }
            .into(),
            "b".to_owned(),
        )];
        let result = Rc::new(XVar {
            prdcns: Prd,
            var: "y".to_owned(),
        })
        .subst_sim(&prod_subst, &cons_subst);

        let expected = Rc::new(
            XVar {
                prdcns: Prd,
                var: "x".to_owned(),
            }
            .into(),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_rc2() {
        let prod_subst = vec![(
            XVar {
                prdcns: Prd,
                var: "x".to_owned(),
            }
            .into(),
            "y".to_owned(),
        )];
        let cons_subst = vec![(
            XVar {
                prdcns: Cns,
                var: "a".to_owned(),
            }
            .into(),
            "b".to_owned(),
        )];
        let result = Rc::new(XVar {
            prdcns: Prd,
            var: "z".to_owned(),
        })
        .subst_sim(&prod_subst, &cons_subst);

        let expected = Rc::new(
            XVar {
                prdcns: Prd,
                var: "z".to_owned(),
            }
            .into(),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_vec() {
        let prod_subst = vec![(
            XVar {
                prdcns: Prd,
                var: "x".to_owned(),
            }
            .into(),
            "y".to_owned(),
        )];
        let cons_subst = vec![(
            XVar {
                prdcns: Cns,
                var: "a".to_owned(),
            }
            .into(),
            "b".to_owned(),
        )];
        let result: Vec<Term<Prd>> = vec![
            XVar {
                prdcns: Prd,
                var: "x".to_owned(),
            },
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
            }
            .into(),
        ]
        .subst_sim(&prod_subst, &cons_subst);

        let expected = vec![
            XVar {
                prdcns: Prd,
                var: "x".to_owned(),
            }
            .into(),
            XVar {
                prdcns: Prd,
                var: "x".to_owned(),
            }
            .into(),
        ];
        assert_eq!(result, expected)
    }
}
