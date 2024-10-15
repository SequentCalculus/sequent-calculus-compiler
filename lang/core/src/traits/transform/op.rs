use crate::syntax::statement::Op;

use super::{Bind, NamingTransformation, TransformState};
use crate::syntax::{
    term::{Prd, XVar},
    Statement, Var,
};

use std::rc::Rc;

impl NamingTransformation for Op {
    type Target = Statement;
    ///N(⊙(p_1, p_2; c)) = bind(p_1)[λa1.bind(p_2)[λa_2.⊙ (a_1, a_2; N(c))]]
    fn transform(self, state: &mut TransformState) -> Statement {
        let cont = Box::new(|var1: Var, state: &mut TransformState| {
            Rc::unwrap_or_clone(self.snd).bind(
                Box::new(|var2: Var, state: &mut TransformState| {
                    Op {
                        fst: Rc::new(
                            XVar {
                                prdcns: Prd,
                                var: var1,
                            }
                            .into(),
                        ),
                        op: self.op,
                        snd: Rc::new(
                            XVar {
                                prdcns: Prd,
                                var: var2,
                            }
                            .into(),
                        ),
                        continuation: self.continuation.transform(state),
                    }
                    .into()
                }),
                state,
            )
        });
        Rc::unwrap_or_clone(self.fst).bind(cont, state)
    }
}

#[cfg(test)]
mod transform_tests {
    use super::NamingTransformation;

    use crate::syntax::{
        statement::{Cut, Op},
        term::{Cns, Literal, Mu, Prd, XVar},
        BinOp,
    };
    use std::rc::Rc;

    fn example_op1() -> Op {
        Op {
            fst: Rc::new(Literal { lit: 1 }.into()),
            op: BinOp::Sum,
            snd: Rc::new(Literal { lit: 2 }.into()),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
    }
    fn example_op2() -> Op {
        Op {
            fst: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                }
                .into(),
            ),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
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
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(Literal { lit: 2 }.into()),
                            consumer: Rc::new(
                                Mu {
                                    prdcns: Cns,
                                    variable: "x1".to_owned(),
                                    statement: Rc::new(
                                        Op {
                                            fst: Rc::new(
                                                XVar {
                                                    prdcns: Prd,
                                                    var: "x0".to_owned(),
                                                }
                                                .into(),
                                            ),
                                            op: BinOp::Sum,
                                            snd: Rc::new(
                                                XVar {
                                                    prdcns: Prd,
                                                    var: "x1".to_owned(),
                                                }
                                                .into(),
                                            ),
                                            continuation: Rc::new(
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

        assert_eq!(result, expected)
    }
    #[test]
    fn transform_op2() {
        let result = example_op2().transform(&mut Default::default());
        let expected = example_op2().into();
        assert_eq!(result, expected)
    }
}
