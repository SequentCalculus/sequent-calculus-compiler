pub mod case;
pub mod clause;
pub mod cocase;
pub mod consumer;
pub mod ctor;
pub mod cut;
pub mod dtor;
pub mod fun;
pub mod ifz;
pub mod lit;
pub mod mu;
pub mod mutilde;
pub mod op;
pub mod producer;

use super::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::{Def, Prog, Statement},
};

impl<T> NamingTransformation for Prog<T> {
    type Target = Prog<T>;
    fn transform(self: Prog<T>, state: &mut TransformState) -> Prog<T> {
        Prog {
            prog_defs: self
                .prog_defs
                .into_iter()
                .map(|def| def.transform(state))
                .collect(),
        }
    }
}

impl<T> NamingTransformation for Def<T> {
    type Target = Def<T>;
    fn transform(self: Def<T>, state: &mut TransformState) -> Def<T> {
        Def {
            name: self.name,
            pargs: self.pargs,
            cargs: self.cargs,
            body: self.body.transform(state),
        }
    }
}

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
mod transform_tests {
    use crate::{
        naming_transformation::NamingTransformation,
        syntax::{BinOp, Covariable, Cut, Def, IfZ, Literal, Op, Prog, Statement, Variable},
    };
    use std::rc::Rc;

    fn example_cut() -> Cut {
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
    fn example_op() -> Op {
        Op {
            fst: Rc::new(Literal { lit: 1 }.into()),
            op: BinOp::Prod,
            snd: Rc::new(Literal { lit: 2 }.into()),
            continuation: Rc::new(
                Covariable {
                    covar: "a".to_owned(),
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
    /*fn example_fun() -> Fun {
        Fun {
            name: "multFast".to_owned(),
            producers: vec![Variable {
                var: "x".to_owned(),
            }
            .into()],

            consumers: vec![Covariable {
                covar: "a".to_owned(),
            }
            .into()],
        }
    }*/
    fn example_done() -> Statement {
        Statement::Done()
    }

    fn example_def1() -> Def<()> {
        Def {
            name: "done".to_owned(),
            pargs: vec![],
            cargs: vec![],
            body: Statement::Done(),
        }
    }
    fn example_def2() -> Def<()> {
        Def {
            name: "cut".to_owned(),
            pargs: vec![("x".to_owned(), ())],
            cargs: vec![("a".to_owned(), ())],
            body: Cut {
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
        }
    }

    fn example_prog1() -> Prog<()> {
        Prog { prog_defs: vec![] }
    }
    fn example_prog2() -> Prog<()> {
        Prog {
            prog_defs: vec![example_def1()],
        }
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

    /* #[test]
    fn transform_fun() {
        let result =
            <Fun as Into<Statement>>::into(example_fun()).transform(&mut Default::default());
        let expected = example_fun().transform(&mut Default::default());
        assert_eq!(result, expected)
    }*/

    #[test]
    fn transform_done() {
        let result = example_done().transform(&mut Default::default());
        let expected = Statement::Done();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_def1() {
        let result = example_def1().transform(&mut Default::default());
        let expected = example_def1();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.pargs, expected.pargs);
        assert_eq!(result.cargs, expected.cargs);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_def2() {
        let result = example_def2().transform(&mut Default::default());
        let expected = example_def2();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.pargs, expected.pargs);
        assert_eq!(result.cargs, expected.cargs);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_prog1() {
        let result = example_prog1().transform(&mut Default::default());
        assert!(result.prog_defs.is_empty())
    }

    #[test]
    fn transform_prog2() {
        let result = example_prog2().transform(&mut Default::default());
        assert_eq!(result.prog_defs.len(), 1);
        let def1 = result.prog_defs.get(0);
        assert!(def1.is_some());
        let def1un = def1.unwrap();
        let ex = example_def1();
        assert_eq!(def1un.name, ex.name);
        assert_eq!(def1un.pargs, ex.pargs);
        assert_eq!(def1un.cargs, ex.cargs);
        assert_eq!(def1un.body, ex.body);
    }
}
