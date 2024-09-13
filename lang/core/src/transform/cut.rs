use super::super::{
    naming_transformation::{bind_many, NamingTransformation, TransformState},
    syntax::{Constructor, Consumer, Cut, Destructor, Producer, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Cut {
    type Target = Statement;
    fn transform(self, state: &mut TransformState) -> Statement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            // N(⟨K(p_i; c_j) | c⟩) = bind(p_i)[λas.bind(c_j)[λbs.⟨K(as; bs) | N(c)⟩]]
            (Producer::Constructor(constructor), consumer) => bind_many(
                constructor.args.into(),
                Box::new(|vars, state: &mut TransformState| {
                    Cut {
                        producer: Rc::new(
                            Constructor {
                                id: constructor.id,
                                args: vars.into_iter().collect(),
                            }
                            .into(),
                        ),
                        consumer: Rc::new(consumer.transform(state)),
                    }
                    .into()
                }),
                state,
            ),
            // N(⟨p | D(p_i; c_j)⟩) = bind(p_i)[λas.bind(c_j)[λbs.⟨N(p) | D(as; bs)⟩]]
            (producer, Consumer::Destructor(destructor)) => bind_many(
                destructor.args.into(),
                Box::new(|args, state: &mut TransformState| {
                    Cut {
                        producer: Rc::new(producer.transform(state)),
                        consumer: Rc::new(
                            Destructor {
                                id: destructor.id,
                                args: args.into_iter().collect(),
                            }
                            .into(),
                        ),
                    }
                    .into()
                }),
                state,
            ),
            // N(⟨p | c⟩) = ⟨N(p) | N(c)⟩
            (producer, consumer) => Cut {
                producer: Rc::new(producer.transform(state)),
                consumer: Rc::new(consumer.transform(state)),
            }
            .into(),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::NamingTransformation,
        syntax::{
            substitution::SubstitutionBinding, Constructor, Covariable, Ctor, Cut, Destructor,
            Dtor, Literal, MuTilde, Variable,
        },
    };
    use std::rc::Rc;

    fn example_ctor() -> Cut {
        Cut {
            producer: Rc::new(
                Constructor {
                    id: Ctor::Cons,
                    args: vec![
                        SubstitutionBinding::ProducerBinding(Literal { lit: 1 }.into()),
                        SubstitutionBinding::ProducerBinding(
                            Constructor {
                                id: Ctor::Nil,
                                args: vec![],
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
                .into(),
            ),
            consumer: Rc::new(
                Covariable {
                    covar: "a".to_owned(),
                }
                .into(),
            ),
        }
    }

    fn example_dtor() -> Cut {
        Cut {
            producer: Rc::new(
                Variable {
                    var: "x".to_owned(),
                }
                .into(),
            ),
            consumer: Rc::new(
                Destructor {
                    id: Dtor::Ap,
                    args: vec![
                        SubstitutionBinding::ProducerBinding(
                            Variable {
                                var: "y".to_owned(),
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
                .into(),
            ),
        }
    }

    fn example_other() -> Cut {
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
    }

    #[test]
    // this illustrates the problem
    fn transform_ctor() {
        let result = example_ctor().transform(&mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            consumer: Rc::new(
                MuTilde {
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Constructor {
                                    id: Ctor::Nil,
                                    args: vec![],
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                MuTilde {
                                    variable: "x1".to_owned(),
                                    statement: Rc::new(
                                        Cut {
                                            producer: Rc::new(
                                                Constructor {
                                                    id: Ctor::Cons,
                                                    args: vec![
                                                        SubstitutionBinding::ProducerBinding(
                                                            Variable {
                                                                var: "x0".to_owned(),
                                                            }
                                                            .into(),
                                                        ),
                                                        SubstitutionBinding::ProducerBinding(
                                                            Variable {
                                                                var: "x1".to_owned(),
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
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();

        assert_eq!(result, expected);
    }

    #[test]
    fn transform_dtor() {
        let result = example_dtor().transform(&mut Default::default());
        let expected = example_dtor().into();
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_other() {
        let result = example_other().transform(&mut Default::default());
        let expected = example_other().into();
        assert_eq!(result, expected);
    }
}
