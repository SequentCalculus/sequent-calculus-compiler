use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Op, Statement, Var, Variable},
};
use std::rc::Rc;

impl NamingTransformation for Op {
    type Target = Statement;
    ///N(⊙(p_1, p_2; c)) = bind(p_1)[λa1.bind(p_2)[λa_2.⊙ (a_1, a_2; c)]]
    fn transform(self, state: &mut TransformState) -> Statement {
        let cont = |var1: Var, state: &mut TransformState| {
            Rc::unwrap_or_clone(self.snd).bind(
                |var2: Var, _: &mut TransformState| {
                    Op {
                        fst: Rc::new(Variable { var: var1 }.into()),
                        op: self.op,
                        snd: Rc::new(Variable { var: var2 }.into()),
                        continuation: self.continuation,
                    }
                    .into()
                },
                state,
            )
        };
        Rc::unwrap_or_clone(self.fst).bind(cont, state)
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::NamingTransformation,
        syntax::{BinOp, Covariable, Cut, Literal, MuTilde, Op, Variable},
    };
    use std::rc::Rc;

    fn example_op1() -> Op {
        Op {
            fst: Rc::new(Literal { lit: 1 }.into()),
            op: BinOp::Sum,
            snd: Rc::new(Literal { lit: 2 }.into()),
            continuation: Rc::new(
                Covariable {
                    covar: "a".to_owned(),
                }
                .into(),
            ),
        }
    }
    fn example_op2() -> Op {
        Op {
            fst: Rc::new(
                Variable {
                    var: "x".to_owned(),
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(
                Variable {
                    var: "y".to_owned(),
                }
                .into(),
            ),
            continuation: Rc::new(
                Covariable {
                    covar: "a".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn transform_op1() {
        let result = example_op1().transform(&mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            consumer: Rc::new(
                MuTilde {
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(Literal { lit: 2 }.into()),
                            consumer: Rc::new(
                                MuTilde {
                                    variable: "x1".to_owned(),
                                    statement: Rc::new(
                                        Op {
                                            fst: Rc::new(
                                                Variable {
                                                    var: "x0".to_owned(),
                                                }
                                                .into(),
                                            ),
                                            op: BinOp::Sum,
                                            snd: Rc::new(
                                                Variable {
                                                    var: "x1".to_owned(),
                                                }
                                                .into(),
                                            ),
                                            continuation: Rc::new(
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

        assert_eq!(result, expected)
    }
    #[test]
    fn transform_op2() {
        let result = example_op2().transform(&mut Default::default());
        let expected = example_op2().into();
        assert_eq!(result, expected)
    }
}
