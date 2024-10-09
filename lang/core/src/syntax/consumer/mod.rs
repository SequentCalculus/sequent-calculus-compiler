use super::{Case, Covar, Covariable, Destructor, MuTilde, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

pub mod case;
pub mod covariable;
pub mod destructor;
pub mod mutilde;

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
            context::ContextBinding, statement::Cut, substitution::SubstitutionBinding, types::Ty,
            Case, Clause, Consumer, Covar, Covariable, Destructor, MuTilde, Producer, Var,
            Variable,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::{collections::HashSet, rc::Rc};

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
                    xtor: "Nil".to_owned(),
                    context: vec![],
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
            id: "Hd".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    Variable {
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    Covariable {
                        covar: "a".to_owned(),
                    }
                    .into(),
                ),
            ],
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
    fn display_mu_tilde() {
        let result = format!("{}", example_mu_tilde());
        let expected = "mutilde x. <x | 'a>".to_owned();
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
    fn display_dest() {
        let result = format!("{}", example_destructor());
        let expected = "Hd(x, 'a)".to_owned();
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
                    xtor: "Nil".to_owned(),
                    context: vec![],
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
            id: "Hd".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    Variable {
                        var: "y".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    Covariable {
                        covar: "b".to_owned(),
                    }
                    .into(),
                ),
            ],
        }
        .into();
        assert_eq!(result, expected)
    }
}
