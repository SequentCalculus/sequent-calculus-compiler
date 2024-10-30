use super::{bind_many, NamingTransformation, TransformState};
use crate::syntax::statement::Cut;
use crate::syntax::{
    term::{Cns, Prd, Term, Xtor},
    Statement,
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
            (Term::Xtor(constructor), consumer) => bind_many(
                constructor.args.into(),
                Box::new(|vars, state: &mut TransformState| {
                    Cut::new(
                        Xtor {
                            prdcns: Prd,
                            id: constructor.id,
                            args: vars.into_iter().collect(),
                            ty: constructor.ty.clone(),
                        },
                        constructor.ty,
                        consumer.transform(state),
                    )
                    .into()
                }),
                state,
            ),
            // N(⟨p | D(p_i; c_j)⟩) = bind(p_i)[λas.bind(c_j)[λbs.⟨N(p) | D(as; bs)⟩]]
            (producer, Term::Xtor(destructor)) => bind_many(
                destructor.args.into(),
                Box::new(|args, state: &mut TransformState| {
                    Cut::new(
                        producer.transform(state),
                        destructor.ty.clone(),
                        Xtor {
                            prdcns: Cns,
                            id: destructor.id,
                            args: args.into_iter().collect(),
                            ty: destructor.ty,
                        },
                    )
                    .into()
                }),
                state,
            ),
            // N(⟨p | c⟩) = ⟨N(p) | N(c)⟩
            (producer, consumer) => Cut {
                producer: Rc::new(producer.transform(state)),
                ty: self.ty,
                consumer: Rc::new(consumer.transform(state)),
            }
            .into(),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::NamingTransformation;
    use crate::syntax::{
        statement::Cut,
        substitution::SubstitutionBinding,
        term::{Cns, Literal, Mu, Prd, XVar, Xtor},
        types::Ty,
    };
    use std::rc::Rc;

    fn example_ctor() -> Cut {
        Cut::new(
            Xtor::ctor(
                "Cons",
                vec![
                    SubstitutionBinding::ProducerBinding {
                        prd: Literal { lit: 1 }.into(),
                        ty: Ty::Int(),
                    },
                    SubstitutionBinding::ProducerBinding {
                        prd: Xtor::ctor("Nil", vec![], Ty::Decl("ListInt".to_owned())).into(),
                        ty: Ty::Decl("ListInt".to_owned()),
                    },
                    SubstitutionBinding::ConsumerBinding {
                        cns: XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                        ty: Ty::Decl("ListInt".to_owned()),
                    },
                ],
                Ty::Decl("ListInt".to_owned()).into(),
            ),
            Ty::Decl("ListInt".to_owned()),
            XVar {
                prdcns: Cns,
                var: "a".to_owned(),
            },
        )
    }

    fn example_dtor() -> Cut {
        Cut {
            producer: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            ty: Ty::Decl("FunIntInt".to_owned()),
            consumer: Rc::new(
                Xtor::dtor(
                    "Ap",
                    vec![
                        SubstitutionBinding::ProducerBinding {
                            prd: XVar {
                                prdcns: Prd,
                                var: "y".to_owned(),
                            }
                            .into(),
                            ty: Ty::Int(),
                        },
                        SubstitutionBinding::ConsumerBinding {
                            cns: XVar {
                                prdcns: Cns,
                                var: "a".to_owned(),
                            }
                            .into(),
                            ty: Ty::Int(),
                        },
                    ],
                    Ty::Decl("FunIntInt".to_owned()),
                )
                .into(),
            ),
        }
    }

    fn example_other() -> Cut {
        Cut {
            producer: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            ty: Ty::Int(),
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn transform_ctor() {
        let result = example_ctor().transform(&mut Default::default());
        let expected = Cut::new(
            Literal { lit: 1 },
            Ty::Int(),
            Mu::tilde_mu(
                "x0",
                Ty::Int(),
                Cut::new(
                    Xtor::ctor("Nil", vec![], Ty::Decl("ListInt".to_owned())),
                    Ty::Decl("ListInt".to_owned()),
                    Mu::tilde_mu(
                        "x1",
                        Ty::Decl("ListInt".to_owned()),
                        Cut::new(
                            Xtor::ctor(
                                "Cons",
                                vec![
                                    SubstitutionBinding::ProducerBinding {
                                        prd: XVar {
                                            prdcns: Prd,
                                            var: "x0".to_owned(),
                                        }
                                        .into(),
                                        ty: Ty::Int(),
                                    },
                                    SubstitutionBinding::ProducerBinding {
                                        prd: XVar {
                                            prdcns: Prd,
                                            var: "x1".to_owned(),
                                        }
                                        .into(),
                                        ty: Ty::Decl("ListInt".to_owned()),
                                    },
                                    SubstitutionBinding::ConsumerBinding {
                                        cns: XVar {
                                            prdcns: Cns,
                                            var: "a".to_owned(),
                                        }
                                        .into(),
                                        ty: Ty::Decl("ListInt".to_owned()),
                                    },
                                ],
                                Ty::Decl("ListInt".to_owned()),
                            ),
                            Ty::Decl("ListInt".to_owned()),
                            XVar {
                                prdcns: Cns,
                                var: "a".to_owned(),
                            },
                        ),
                    ),
                ),
            ),
        )
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
