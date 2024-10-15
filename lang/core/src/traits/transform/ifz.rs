use super::{Bind, NamingTransformation, TransformState};
use crate::syntax::statement::IfZ;
use crate::syntax::{
    term::{Prd, XVar},
    Statement,
};
use std::rc::Rc;

impl NamingTransformation for IfZ {
    type Target = Statement;
    ///N(ifz(p, s_1, s_2)) = bind(p)[λa.ifz(a, N(s_1), N(s_2))]
    fn transform(self, state: &mut TransformState) -> Statement {
        let then_transformed = self.thenc.transform(state);
        let else_transformed = self.elsec.transform(state);
        let cont = Box::new(|var, _: &mut TransformState| {
            IfZ {
                ifc: Rc::new(XVar { prdcns: Prd, var }.into()),
                thenc: then_transformed,
                elsec: else_transformed,
            }
            .into()
        });

        Rc::unwrap_or_clone(self.ifc).bind(cont, state)
    }
}

#[cfg(test)]
mod transform_tests {
    use super::NamingTransformation;
    use crate::syntax::{
        statement::{Cut, IfZ},
        term::{Cns, Literal, Mu, Prd, XVar},
        Statement,
    };
    use std::rc::Rc;

    fn example_ifz1() -> IfZ {
        IfZ {
            ifc: Rc::new(Literal { lit: 1 }.into()),
            thenc: Rc::new(
                Cut {
                    producer: Rc::new(Literal { lit: 1 }.into()),
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
            elsec: Rc::new(Statement::Done()),
        }
    }
    fn example_ifz2() -> IfZ {
        IfZ {
            ifc: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            thenc: Rc::new(Statement::Done()),
            elsec: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
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
    }

    #[test]
    fn transform_ifz1() {
        let result = example_ifz1().transform(&mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        IfZ {
                            ifc: Rc::new(
                                XVar {
                                    prdcns: Prd,
                                    var: "x0".to_owned(),
                                }
                                .into(),
                            ),
                            thenc: Rc::new(
                                Cut {
                                    producer: Rc::new(Literal { lit: 1 }.into()),
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
                            elsec: Rc::new(Statement::Done()),
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
    fn transform_ifz2() {
        let result = example_ifz2().transform(&mut Default::default());
        let expected = example_ifz2().into();
        assert_eq!(result, expected)
    }
}
