use super::{Cns, Prd, PrdCns};
use crate::{
    syntax::{stringify_and_join, Clause, Covar, Var},
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt};

// Cocase
//
//

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

impl<T: PrdCns> FreeV for XCase<T> {
    fn free_vars(&self) -> HashSet<Var> {
        self.clauses.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.clauses.free_covars()
    }
}

impl From<Cocase> for Producer {
    fn from(value: Cocase) -> Self {
        Producer::Cocase(value)
    }
}

impl Subst for Cocase {
    type Target = Cocase;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Cocase {
            cocases: self.cocases.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod cocase_test {
    use crate::{
        syntax::{
            context::ContextBinding, statement::Cut, types::Ty, Clause, Cocase, Consumer, Covar,
            Covariable, Producer, Var, Variable,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_cocase() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: "Hd".to_owned(),
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
                    xtor: "Tl".to_owned(),
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
    fn display_cocase() {
        let result = format!("{}", example_cocase());
        let expected =
            "cocase { Hd(x : Int, 'a :cnt Int) => <x | 'a>, Tl() => <x | 'a> }".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_cocase() {
        let result = example_cocase().free_vars();
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
    fn subst_cocase() {
        let result = example_cocase().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Cocase {
            cocases: vec![
                Clause {
                    xtor: "Hd".to_owned(),
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
                Clause {
                    xtor: "Tl".to_owned(),
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
            ],
        };
        assert_eq!(result, expected)
    }
}
