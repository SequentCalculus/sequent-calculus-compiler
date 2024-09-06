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

pub fn transform_def(def: Def) -> Def {
    let mut initial_state = TransformState {
        used_vars: def.pargs.iter().map(|(var, _)| var).cloned().collect(),
        used_covars: def.cargs.iter().map(|(covar, _)| covar).cloned().collect(),
    };

    Def {
        name: def.name,
        pargs: def.pargs,
        cargs: def.cargs,
        body: def.body.transform(&mut initial_state),
    }
}

pub fn transform_prog(prog: Prog) -> Prog {
    Prog {
        prog_defs: prog.prog_defs.into_iter().map(transform_def).collect(),
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
        syntax::{BinOp, Covariable, Cut, Def, Fun, IfZ, Literal, Op, Prog, Statement, Variable},
        transform::{transform_def, transform_prog},
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

    fn example_fun() -> Fun {
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
    }

    fn example_done() -> Statement {
        Statement::Done()
    }

    fn example_def1() -> Def {
        Def {
            name: "done".to_owned(),
            pargs: vec![],
            cargs: vec![],
            body: Statement::Done(),
        }
    }
    fn example_def2() -> Def {
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

    fn example_prog1() -> Prog {
        Prog { prog_defs: vec![] }
    }
    fn example_prog2() -> Prog {
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

    #[test]
    fn transform_def1() {
        let result = transform_def(example_def1());
        let expected = example_def1();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.pargs, expected.pargs);
        assert_eq!(result.cargs, expected.cargs);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_def2() {
        let result = transform_def(example_def2());
        let expected = example_def2();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.pargs, expected.pargs);
        assert_eq!(result.cargs, expected.cargs);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_prog1() {
        let result = transform_prog(example_prog1());
        assert!(result.prog_defs.is_empty())
    }

    #[test]
    fn transform_prog2() {
        let result = transform_prog(example_prog2());
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
