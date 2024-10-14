use super::{Covar, Producer, Var};
use crate::{
    syntax::{
        stringify_and_join,
        term::{Cns, Prd, Term},
        Clause,
    },
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt};

// Cocase
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cocase {
    pub cocases: Vec<Clause>,
}

impl std::fmt::Display for Cocase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses_joined: String = stringify_and_join(&self.cocases);
        write!(f, "cocase {{ {} }}", clauses_joined)
    }
}

impl FreeV for Cocase {
    fn free_vars(&self) -> HashSet<Var> {
        self.cocases.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.cocases.free_covars()
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
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
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
            context::ContextBinding,
            statement::Cut,
            term::{Cns, Prd, Term, XVar},
            types::Ty,
            Clause, Cocase, Covar, Covariable, Var, Variable,
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
