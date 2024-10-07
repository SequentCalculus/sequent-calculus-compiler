use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};
pub mod cocase;
pub mod constructor;
pub mod literal;
pub mod mu;
pub mod variable;
pub use cocase::XCase;
pub use constructor::Xtor;
pub use literal::Literal;
pub use mu::Mu;
pub use variable::XVar;

pub struct Prd;
pub struct Cns;

pub trait PrdCns {
    fn is_prd(&self) -> bool;
}

impl PrdCns for Prd {
    fn is_prd(&self) -> bool {
        true
    }
}

impl PrdCns for Cns {
    fn is_prd(&self) -> bool {
        false
    }
}

// Term
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term<T: PrdCns> {
    XVar(XVar<T>),
    Literal(Literal<T>),
    Mu(Mu<T>),
    Xtor(Xtor<T>),
    XCase(XCase<T>),
}

impl std::fmt::Display for Term<Prd> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::XVar(v) => v.fmt(f),
            Term::Literal(i) => i.fmt(f),
            Term::Mu(m) => m.fmt(f),
            Term::Xtor(c) => c.fmt(f),
            Term::XCase(c) => c.fmt(f),
        }
    }
}
impl std::fmt::Display for Term<Cns> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::XVar(v) => v.fmt(f),
            Term::Literal(i) => i.fmt(f),
            Term::Mu(m) => m.fmt(f),
            Term::Xtor(c) => c.fmt(f),
            Term::XCase(c) => c.fmt(f),
        }
    }
}

impl FreeV for Producer {
    fn free_vars(self: &Producer) -> HashSet<crate::syntax::Var> {
        match self {
            Producer::XVar(v) => v.free_vars(),
            Producer::Literal(l) => l.free_vars(),
            Producer::Mu(m) => m.free_vars(),
            Producer::Constructor(c) => c.free_vars(),
            Producer::Cocase(c) => c.free_vars(),
        }
    }

    fn free_covars(self: &Producer) -> HashSet<Covar> {
        match self {
            Producer::XVar(v) => v.free_covars(),
            Producer::Literal(l) => l.free_covars(),
            Producer::Mu(m) => m.free_covars(),
            Producer::Constructor(c) => c.free_covars(),
            Producer::Cocase(c) => c.free_covars(),
        }
    }
}

impl Subst for Producer {
    type Target = Producer;
    fn subst_sim(
        self: &Producer,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Producer {
        match self {
            Producer::XVar(v) => v.subst_sim(prod_subst, cons_subst),
            Producer::Literal(l) => l.subst_sim(prod_subst, cons_subst).into(),
            Producer::Mu(m) => m.subst_sim(prod_subst, cons_subst).into(),
            Producer::Constructor(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Producer::Cocase(c) => c.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}

#[cfg(test)]
mod producer_tests {
    use crate::{
        syntax::{
            context::ContextBinding, statement::Cut, substitution::SubstitutionBinding, types::Ty,
            Clause, Cocase, Constructor, Consumer, Covar, Covariable, Literal, Mu, Producer, Var,
            XVar,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_var() -> Producer {
        XVar {
            var: "x".to_owned(),
        }
        .into()
    }
    fn example_lit() -> Producer {
        Literal { lit: 1 }.into()
    }
    fn example_mu() -> Producer {
        Mu {
            covariable: "a".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
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
    fn example_constructor() -> Producer {
        Constructor {
            id: "Cons".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        var: "xs".to_owned(),
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
    fn example_cocase() -> Producer {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: "Fst".to_owned(),
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
                                XVar {
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
                    xtor: "Snd".to_owned(),
                    context: vec![],
                    rhs: Rc::new(
                        Cut {
                            producer: Rc::new(
                                XVar {
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

    fn example_prodsubst() -> Vec<(Producer, Var)> {
        vec![(
            XVar {
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
    fn display_var() {
        let result = format!("{}", example_var());
        let expected = "x".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_lit() {
        let result = format!("{}", example_lit());
        let expected = "1".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_mu() {
        let result = format!("{}", example_mu());
        let expected = "mu 'a. <x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_const() {
        let result = format!("{}", example_constructor());
        let expected = "Cons(x, xs, 'a)".to_owned();
        assert_eq!(result, expected)
    }
    #[test]
    fn display_cocase() {
        let result = format!("{}", example_cocase());
        let expected =
            "cocase { Fst(x : Int, 'a :cnt Int) => <x | 'a>, Snd() => <x | 'a> }".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_var() {
        let result = example_var().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_lit() {
        let result = example_lit().free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_mu() {
        let result = example_var().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_const() {
        let result = example_constructor().free_vars();
        let expected = HashSet::from(["x".to_owned(), "xs".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_cocase() {
        let result = example_cocase().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_var() {
        let result = example_var().free_covars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_lit() {
        let result = example_lit().free_covars();
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
    fn free_covars_const() {
        let result = example_constructor().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_cocase() {
        let result = example_cocase().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_var() {
        let result = example_var().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar {
            var: "y".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_lit() {
        let result = example_lit().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Literal { lit: 1 }.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_mu() {
        let result = example_mu().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            var: "y".to_owned(),
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
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_const() {
        let result = example_constructor().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Constructor {
            id: "Cons".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        var: "y".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        var: "xs".to_owned(),
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

    #[test]
    fn subst_cocase() {
        let result = example_cocase().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Cocase {
            cocases: vec![
                Clause {
                    xtor: "Fst".to_owned(),
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
                                XVar {
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
                    xtor: "Snd".to_owned(),
                    context: vec![],
                    rhs: Rc::new(
                        Cut {
                            producer: Rc::new(
                                XVar {
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
        }
        .into();
        assert_eq!(result, expected)
    }
}
