use super::{Case, Covar, Covariable, Destructor, MuTilde, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Consumer
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Consumer {
    Covariable(Covariable),
    MuTilde(MuTilde),
    Case(Case),
    Destructor(Destructor),
}

impl std::fmt::Display for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Consumer::Covariable(cv) => cv.fmt(f),
            Consumer::MuTilde(m) => m.fmt(f),
            Consumer::Case(case) => case.fmt(f),
            Consumer::Destructor(d) => d.fmt(f),
        }
    }
}

impl FreeV for Consumer {
    fn free_vars(self: &Consumer) -> HashSet<Var> {
        match self {
            Consumer::Covariable(cv) => cv.free_vars(),
            Consumer::MuTilde(m) => m.free_vars(),
            Consumer::Case(pts) => pts.free_vars(),
            Consumer::Destructor(d) => d.free_vars(),
        }
    }

    fn free_covars(self: &Consumer) -> HashSet<Covar> {
        match self {
            Consumer::Covariable(covar) => covar.free_covars(),
            Consumer::MuTilde(m) => m.free_covars(),
            Consumer::Case(c) => c.free_covars(),
            Consumer::Destructor(d) => d.free_covars(),
        }
    }
}

impl Subst for Consumer {
    type Target = Consumer;
    fn subst_sim(
        self: &Consumer,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Consumer {
        match self {
            Consumer::Covariable(covar) => covar.subst_sim(prod_subst, cons_subst),
            Consumer::MuTilde(m) => m.subst_sim(prod_subst, cons_subst).into(),
            Consumer::Case(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Consumer::Destructor(d) => d.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}

#[cfg(test)]
mod consumer_tests {
    use crate::{
        syntax::{
            Case, Clause, Consumer, Covar, Covariable, Ctor, Cut, Destructor, Dtor, MuTilde,
            Producer, Var, Variable,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_covar() -> Consumer {
        Covariable {
            covar: "a".to_owned(),
        }
        .into()
    }
    fn example_mu_tilde() -> Consumer {
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
        .into()
    }
    fn example_case() -> Consumer {
        Case {
            cases: vec![
                Clause {
                    xtor: Ctor::Nil,
                    vars: vec![],
                    covars: vec![],
                    rhs: Rc::new(
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
                },
                Clause {
                    xtor: Ctor::Cons,
                    vars: vec!["x".to_owned(), "xs".to_owned()],
                    covars: vec!["a".to_owned()],
                    rhs: Rc::new(
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
                },
            ],
        }
        .into()
    }

    fn example_destructor() -> Consumer {
        Destructor {
            id: Dtor::Hd,
            producers: vec![Variable {
                var: "x".to_owned(),
            }
            .into()],
            consumers: vec![Covariable {
                covar: "a".to_owned(),
            }
            .into()],
        }
        .into()
    }

    fn example_prodsubst() -> Vec<(Producer, Var)> {
        vec![(
            Variable {
                var: "y".to_owned(),
            }
            .into(),
            "x".to_owned(),
        )]
    }
    fn example_conssubst() -> Vec<(Consumer, Covar)> {
        vec![(
            Covariable {
                covar: "b".to_owned(),
            }
            .into(),
            "a".to_owned(),
        )]
    }

    #[test]
    fn display_covar() {
        let result = format!("{}", example_covar());
        let expected = "a".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_mu_tilde() {
        let result = format!("{}", example_mu_tilde());
        let expected = "mutilde x. <x | a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_case() {
        let result = format!("{}", example_case());
        let expected = "case { Nil(; ) => <x | a>, Cons(x, xs; a) => <x | a> }".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_dest() {
        let result = format!("{}", example_destructor());
        let expected = "hd(x; a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_covar() {
        let result = example_covar().free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_mu_tilde() {
        let result = example_mu_tilde().free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_case() {
        let result = example_case().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_dest() {
        let result = example_destructor().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_covar() {
        let result = example_covar().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_mu_tilde() {
        let result = example_mu_tilde().free_covars();
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
    fn free_covars_dest() {
        let result = example_destructor().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_covar() {
        let result = example_covar().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Covariable {
            covar: "b".to_owned(),
        }
        .into();
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
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_case() {
        let result = example_case().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Case {
            cases: vec![
                Clause {
                    xtor: Ctor::Nil,
                    vars: vec![],
                    covars: vec![],
                    rhs: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Variable {
                                    var: "y".to_owned(),
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
                },
                Clause {
                    xtor: Ctor::Cons,
                    vars: vec!["x0".to_owned(), "x1".to_owned()],
                    covars: vec!["a0".to_owned()],
                    rhs: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Variable {
                                    var: "x0".to_owned(),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                Covariable {
                                    covar: "a0".to_owned(),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                },
            ],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_dest() {
        let result = example_destructor().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Destructor {
            id: Dtor::Hd,
            producers: vec![Variable {
                var: "y".to_owned(),
            }
            .into()],
            consumers: vec![Covariable {
                covar: "b".to_owned(),
            }
            .into()],
        }
        .into();
        assert_eq!(result, expected)
    }
}
