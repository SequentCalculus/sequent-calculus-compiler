use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{
        Constructor, Consumer, Covar, Covariable, Cut, Destructor, Producer, Statement, Var,
        Variable,
    },
};
use std::rc::Rc;

impl NamingTransformation for Cut {
    type Target = Statement;
    fn transform(self, st: &mut TransformState) -> Statement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            //N (⟨K (pi ; c j ) | c⟩) = bind(pi ) [λas.bind(c j ) [λbs.⟨K (as; bs) | N (c)⟩]]
            (Producer::Constructor(ctor), cons) => {
                let cont = |ns: Vec<Var>| {
                    |_: &mut TransformState| {
                        Bind::bind_many(ctor.consumers, |bs: Vec<Covar>| {
                            |st: &mut TransformState| {
                                Cut {
                                    producer: Rc::new(
                                        Constructor {
                                            id: ctor.id,
                                            producers: ns
                                                .into_iter()
                                                .map(|n| Variable { var: n }.into())
                                                .collect(),
                                            consumers: bs
                                                .into_iter()
                                                .map(|b| Covariable { covar: b }.into())
                                                .collect(),
                                        }
                                        .into(),
                                    ),
                                    consumer: Rc::new(cons.transform(st)),
                                }
                                .into()
                            }
                        })
                    }
                };
                Bind::bind_many(ctor.producers, cont)
            }
            //N (⟨μα .s | D (pi ; c j )⟩) = ⟨N (μα .s) | N (D (pi ; c j ))⟩
            (Producer::Mu(mu), Consumer::Destructor(dest)) => Cut {
                producer: Rc::new(mu.transform(st).into()),
                consumer: Rc::new(dest.transform(st).into()),
            }
            .into(),
            //N (⟨p | D (pi ; c j )⟩) = bind(pi ) [λas.bind(c j ) [λbs.⟨N (p) | D (as; bs)⟩]]
            (prod, Consumer::Destructor(dest)) => {
                let cont = |ns: Vec<Var>| {
                    |_: &mut TransformState| {
                        Bind::bind_many(dest.consumers, |bs: Vec<Covar>| {
                            |st: &mut TransformState| {
                                Cut {
                                    producer: Rc::new(prod.transform(st)),
                                    consumer: Rc::new(
                                        Destructor {
                                            id: dest.id,
                                            producers: ns
                                                .into_iter()
                                                .map(|n| Variable { var: n }.into())
                                                .collect(),
                                            consumers: bs
                                                .into_iter()
                                                .map(|b| Covariable { covar: b }.into())
                                                .collect(),
                                        }
                                        .into(),
                                    ),
                                }
                                .into()
                            }
                        })
                    }
                };

                Bind::bind_many(dest.producers, cont)
            }
            //N (⟨p | c⟩) = ⟨N (p) | N (c)⟩
            (prod, cons) => Cut {
                producer: Rc::new(prod.transform(st)),
                consumer: Rc::new(cons.transform(st)),
            }
            .into(),
        }
    }
}

/*
#[cfg(test)]
mod transform_tests {
    use super::Cut;
    use crate::naming_transformation::NamingTransformation;

    fn example_ctor() -> Cut {
        todo!("not implemented")
    }
    fn example_mu_dtor() -> Cut {
        todo!("not implemented")
    }
    fn example_dtor() -> Cut {
        todo!("not implemented")
    }
    fn example_other() -> Cut {
        todo!("not implemented")
    }

    #[test]
    fn transform_ctor() {
        let result = example_ctor().transform(&mut Default::default());
        let expected = todo!("not implemented");
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_mu_dtor() {
        let result = example_mu_dtor().transform(&mut Default::default());
        let expected = todo!("not implemented");
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_dtor() {
        let result = example_dtor().transform(&mut Default::default());
        let expected = todo!("not implemented");
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_other() {
        let result = example_other().transform(&mut Default::default());
        let expected = todo!("not implemented");
        assert_eq!(result, expected);
    }
}*/
