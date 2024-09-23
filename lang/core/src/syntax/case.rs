use super::{stringify_and_join, Clause, Consumer, Covar, Ctor, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Case
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    pub cases: Vec<Clause<Ctor>>,
}

impl std::fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses_joined: String = stringify_and_join(&self.cases);
        write!(f, "case {{ {} }}", clauses_joined)
    }
}

impl FreeV for Case {
    fn free_vars(&self) -> HashSet<Var> {
        self.cases.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.cases.free_covars()
    }
}

impl From<Case> for Consumer {
    fn from(value: Case) -> Self {
        Consumer::Case(value)
    }
}

impl Subst for Case {
    type Target = Case;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Case {
            cases: self.cases.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod case_test {
    use crate::{
        syntax::{
            context::ContextBinding, types::Ty, Case, Clause, Consumer, Covar, Covariable, Ctor,
            Cut, Producer, Var, Variable,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_case() -> Case {
        Case {
            cases: vec![
                Clause {
                    xtor: Ctor::Nil,
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
                    xtor: Ctor::Cons,
                    context: vec![
                        ContextBinding::VarBinding {
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::CovarBinding {
                            covar: "a".to_owned(),
                            ty: Ty::Decl("ListInt".to_owned()),
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
    fn display_case() {
        let result = format!("{}", example_case());
        let expected =
            "case { Nil() => <x | 'a>, Cons(x : Int, 'a :cnt ListInt) => <x | 'a> }".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_case() {
        let result = example_case().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
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
        let expected = Case {
            cases: vec![
                Clause {
                    xtor: Ctor::Nil,
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
                    xtor: Ctor::Cons,
                    context: vec![
                        ContextBinding::VarBinding {
                            var: "x0".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::CovarBinding {
                            covar: "a0".to_owned(),
                            ty: Ty::Decl("ListInt".to_owned()),
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
        };
        assert_eq!(result, expected)
    }
}
