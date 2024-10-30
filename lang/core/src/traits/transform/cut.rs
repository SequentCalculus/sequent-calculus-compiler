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
                    Cut {
                        producer: Rc::new(
                            Xtor {
                                prdcns: Prd,
                                id: constructor.id,
                                args: vars.into_iter().collect(),
                            }
                            .into(),
                        ),
                        ty: self.ty,
                        consumer: Rc::new(consumer.transform(state)),
                    }
                    .into()
                }),
                state,
            ),
            // N(⟨p | D(p_i; c_j)⟩) = bind(p_i)[λas.bind(c_j)[λbs.⟨N(p) | D(as; bs)⟩]]
            (producer, Term::Xtor(destructor)) => bind_many(
                destructor.args.into(),
                Box::new(|args, state: &mut TransformState| {
                    Cut {
                        producer: Rc::new(producer.transform(state)),
                        ty: self.ty,
                        consumer: Rc::new(
                            Xtor {
                                prdcns: Cns,
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
                ty: self.ty,
                consumer: Rc::new(consumer.transform(state)),
            }
            .into(),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::{NamingTransformation, TransformState};
    use crate::syntax::{
        context::ContextBinding,
        declaration::{Data, TypeDeclaration, XtorSig},
        statement::Cut,
        substitution::SubstitutionBinding,
        term::{Cns, Literal, Mu, Prd, XVar, Xtor},
        types::Ty,
    };
    use std::rc::Rc;

    fn example_ctor() -> Cut {
        Cut {
            producer: Rc::new(
                Xtor {
                    prdcns: Prd,
                    id: "Cons".to_owned(),
                    args: vec![
                        SubstitutionBinding::ProducerBinding(Literal { lit: 1 }.into()),
                        SubstitutionBinding::ProducerBinding(
                            Xtor {
                                prdcns: Prd,
                                id: "Nil".to_owned(),
                                args: vec![],
                            }
                            .into(),
                        ),
                        SubstitutionBinding::ConsumerBinding(
                            XVar {
                                prdcns: Cns,
                                var: "a".to_owned(),
                            }
                            .into(),
                        ),
                    ],
                }
                .into(),
            ),
            ty: Ty::Decl("ListInt".to_owned()),
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
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
                Xtor {
                    prdcns: Cns,
                    id: "Ap".to_owned(),
                    args: vec![
                        SubstitutionBinding::ProducerBinding(
                            XVar {
                                prdcns: Prd,
                                var: "y".to_owned(),
                            }
                            .into(),
                        ),
                        SubstitutionBinding::ConsumerBinding(
                            XVar {
                                prdcns: Cns,
                                var: "a".to_owned(),
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
    // this illustrates the problem
    fn transform_ctor() {
        let mut state = TransformState::default();
        state.data_decls.push(TypeDeclaration {
            dat: Data,
            name: "ListInt".to_owned(),
            xtors: vec![
                XtorSig {
                    xtor: Data,
                    name: "Nil".to_owned(),
                    args: vec![],
                },
                XtorSig {
                    xtor: Data,
                    name: "Cons".to_owned(),
                    args: vec![
                        ContextBinding::VarBinding {
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::VarBinding {
                            var: "xs".to_owned(),
                            ty: Ty::Decl("ListInt".to_owned()),
                        },
                    ],
                },
            ],
        });
        let result = example_ctor().transform(&mut state);
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            ty: Ty::Int(),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    var_ty: Ty::Int(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Xtor {
                                    prdcns: Prd,
                                    id: "Nil".to_owned(),
                                    args: vec![],
                                }
                                .into(),
                            ),
                            ty: Ty::Decl("ListInt".to_owned()),
                            consumer: Rc::new(
                                Mu {
                                    prdcns: Cns,
                                    variable: "x1".to_owned(),
                                    var_ty: Ty::Decl("ListInt".to_owned()),
                                    statement: Rc::new(
                                        Cut {
                                            producer: Rc::new(
                                                Xtor {
                                                    prdcns: Prd,
                                                    id: "Cons".to_owned(),
                                                    args: vec![
                                                        SubstitutionBinding::ProducerBinding(
                                                            XVar {
                                                                prdcns: Prd,
                                                                var: "x0".to_owned(),
                                                            }
                                                            .into(),
                                                        ),
                                                        SubstitutionBinding::ProducerBinding(
                                                            XVar {
                                                                prdcns: Prd,
                                                                var: "x1".to_owned(),
                                                            }
                                                            .into(),
                                                        ),
                                                        SubstitutionBinding::ConsumerBinding(
                                                            XVar {
                                                                prdcns: Cns,
                                                                var: "a".to_owned(),
                                                            }
                                                            .into(),
                                                        ),
                                                    ],
                                                }
                                                .into(),
                                            ),
                                            ty: Ty::Decl("ListInt".to_owned()),
                                            consumer: Rc::new(
                                                XVar {
                                                    prdcns: Cns,
                                                    var: "a".to_owned(),
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
