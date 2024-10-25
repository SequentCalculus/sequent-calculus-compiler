use super::{NamingTransformation, TransformState};
use crate::syntax::Statement;

impl NamingTransformation for Statement {
    type Target = Statement;
    fn transform(self: Statement, state: &mut TransformState) -> Statement {
        match self {
            Statement::Cut(cut) => cut.transform(state),
            Statement::Op(op) => op.transform(state),
            Statement::IfZ(ifz) => ifz.transform(state),
            Statement::Fun(fun) => fun.transform(state),
            Statement::Done() => Statement::Done(),
        }
    }
}

#[cfg(test)]
mod statement_tests {
    use super::NamingTransformation;
    use crate::syntax::{
        statement::{Cut, Fun, IfZ, Op},
        substitution::SubstitutionBinding,
        term::{Cns, Literal, Prd, XVar},
        types::Ty,
        BinOp, Statement,
    };
    use std::rc::Rc;

    fn example_cut() -> Cut {
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
    fn example_op() -> Op {
        Op {
            fst: Rc::new(Literal { lit: 1 }.into()),
            op: BinOp::Prod,
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

    fn example_ifz() -> IfZ {
        IfZ {
            ifc: Rc::new(Literal { lit: 0 }.into()),
            thenc: Rc::new(Statement::Done()),
            elsec: Rc::new(Statement::Done()),
        }
    }

    fn example_fun() -> Fun {
        Fun {
            name: "multFast".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
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
    }

    fn example_done() -> Statement {
        Statement::Done()
    }

    #[test]
    fn transform_cut() {
        let result =
            <Cut as Into<Statement>>::into(example_cut()).transform(&mut Default::default());
        let expected = example_cut().transform(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_op() {
        let result = <Op as Into<Statement>>::into(example_op()).transform(&mut Default::default());
        let expected = example_op().transform(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_ifz() {
        let result =
            <IfZ as Into<Statement>>::into(example_ifz()).transform(&mut Default::default());
        let expected = example_ifz().transform(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_fun() {
        let result =
            <Fun as Into<Statement>>::into(example_fun()).transform(&mut Default::default());
        let expected = example_fun().transform(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_done() {
        let result = example_done().transform(&mut Default::default());
        let expected = Statement::Done();
        assert_eq!(result, expected)
    }
}
