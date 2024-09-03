//! Compiling a program from the source language `Fun` to the intermediate language `Core`.

use crate::definition::{CompileState, CompileWithCont};
use fun::syntax::Covariable;

pub fn compile_def<T>(def: fun::program::Def<T>) -> core::syntax::Def<T> {
    let mut initial_state: CompileState = CompileState {
        covars: def.cont.iter().map(|(covar, _)| covar).cloned().collect(),
    };
    let new_covar = initial_state.free_covar_from_state();
    let body = def.body.compile_with_cont(
        core::syntax::Covariable {
            covar: new_covar.clone(),
        }
        .into(),
        &mut initial_state,
    );

    let mut new_cont: Vec<(Covariable, T)> = def.cont;
    new_cont.push((new_covar, def.ret_ty));

    core::syntax::Def {
        name: def.name,
        pargs: def.args,
        cargs: new_cont,
        body,
    }
}

pub fn compile_prog<T>(prog: fun::program::Prog<T>) -> core::syntax::Prog<T> {
    core::syntax::Prog {
        prog_defs: prog
            .prog_defs
            .into_iter()
            .map(|def| compile_def(def))
            .collect(),
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::program::{compile_def, compile_prog};
    use fun::{
        program::{Def, Prog},
        syntax::terms::Term,
    };
    use std::rc::Rc;

    fn example_def1() -> Def<()> {
        Def {
            name: "main".to_owned(),
            args: vec![],
            cont: vec![("a".to_owned(), ())],
            body: Term::Lit(1),
            ret_ty: (),
        }
    }
    fn example_def2() -> Def<()> {
        Def {
            name: "id".to_owned(),
            args: vec![("x".to_owned(), ())],
            cont: vec![],
            body: Term::Var("x".to_owned()),
            ret_ty: (),
        }
    }

    fn example_prog1() -> Prog<()> {
        Prog { prog_defs: vec![] }
    }

    fn example_prog2() -> Prog<()> {
        Prog {
            prog_defs: vec![example_def1(), example_def2()],
        }
    }

    #[test]
    fn compile_def1() {
        let result = compile_def(example_def1());
        let expected = core::syntax::Def {
            name: "main".to_owned(),
            pargs: vec![],
            cargs: vec![("a".to_owned(), ()), ("a0".to_owned(), ())],
            body: core::syntax::Cut {
                producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                consumer: Rc::new(
                    core::syntax::Covariable {
                        covar: "a0".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        };
        assert_eq!(result.name, expected.name);
        assert_eq!(result.pargs, expected.pargs);
        assert_eq!(result.cargs, expected.cargs);
        assert_eq!(result.body, expected.body);
    }
    #[test]
    fn compile_def2() {
        let result = compile_def(example_def2());
        let expected = core::syntax::Def {
            name: "id".to_owned(),
            pargs: vec![("x".to_owned(), ())],
            cargs: vec![("a0".to_owned(), ())],
            body: core::syntax::Cut {
                producer: Rc::new(
                    core::syntax::Variable {
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                consumer: Rc::new(
                    core::syntax::Covariable {
                        covar: "a0".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        };
        assert_eq!(result.name, expected.name);
        assert_eq!(result.pargs, expected.pargs);
        assert_eq!(result.cargs, expected.cargs);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn compile_prog1() {
        let result = compile_prog(example_prog1());
        assert!(result.prog_defs.is_empty())
    }

    #[test]
    fn compile_prog2() {
        let result = compile_prog(example_prog2());
        assert_eq!(result.prog_defs.len(), 2);
        let def1 = result.prog_defs.get(0).unwrap();
        let def2 = result.prog_defs.get(1).unwrap();
        let expected1 = core::syntax::Def {
            name: "main".to_owned(),
            pargs: vec![],
            cargs: vec![("a".to_owned(), ()), ("a0".to_owned(), ())],
            body: core::syntax::Cut {
                producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                consumer: Rc::new(
                    core::syntax::Covariable {
                        covar: "a0".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        };
        let expected2 = core::syntax::Def {
            name: "id".to_owned(),
            pargs: vec![("x".to_owned(), ())],
            cargs: vec![("a0".to_owned(), ())],
            body: core::syntax::Cut {
                producer: Rc::new(
                    core::syntax::Variable {
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                consumer: Rc::new(
                    core::syntax::Covariable {
                        covar: "a0".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        };
        assert_eq!(def1.name, expected1.name);
        assert_eq!(def1.pargs, expected1.pargs);
        assert_eq!(def1.cargs, expected1.cargs);
        assert_eq!(def1.body, expected1.body);
        assert_eq!(def2.name, expected2.name);
        assert_eq!(def2.pargs, expected2.pargs);
        assert_eq!(def2.cargs, expected2.cargs);
        assert_eq!(def2.body, expected2.body);
    }
}
