use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{IfZ, Statement, Variable},
};
use std::rc::Rc;

impl NamingTransformation for IfZ {
    type Target = Statement;
    ///N (ifz(p, s1 , s2 )) = bind(p) [λa.ifz(a, N (s 1), N (s 2 ))]
    fn transform(self, st: &mut TransformState) -> Statement {
        let then_trans = self.thenc.transform(st);
        let else_trans = self.elsec.transform(st);
        let cont = |a| {
            |_: &mut TransformState| {
                IfZ {
                    ifc: Rc::new(Variable { var: a }.into()),
                    thenc: then_trans,
                    elsec: else_trans,
                }
                .into()
            }
        };

        Rc::unwrap_or_clone(self.ifc).bind(cont, st)
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::NamingTransformation,
        syntax::{Covariable, Cut, IfZ, Literal, MuTilde, Statement, Variable},
    };
    use std::rc::Rc;

    fn example_ifz1() -> IfZ {
        IfZ {
            ifc: Rc::new(Literal { lit: 1 }.into()),
            thenc: Rc::new(
                Cut {
                    producer: Rc::new(Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        Covariable {
                            covar: "a".to_owned(),
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
                Variable {
                    var: "x".to_owned(),
                }
                .into(),
            ),
            thenc: Rc::new(Statement::Done()),
            elsec: Rc::new(
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
    }

    #[test]
    fn transform_ifz1() {
        let result = example_ifz1().transform(&mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            consumer: Rc::new(
                MuTilde {
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        IfZ {
                            ifc: Rc::new(
                                Variable {
                                    var: "x0".to_owned(),
                                }
                                .into(),
                            ),
                            thenc: Rc::new(
                                Cut {
                                    producer: Rc::new(Literal { lit: 1 }.into()),
                                    consumer: Rc::new(
                                        Covariable {
                                            covar: "a".to_owned(),
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
