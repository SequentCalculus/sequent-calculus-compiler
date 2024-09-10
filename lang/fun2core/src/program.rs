//! Compiling a program from the source language `Fun` to the intermediate language `Core`.

use crate::definition::{CompileState, CompileWithCont};
use fun::syntax::{
    context::{context_covars, context_vars},
    Covariable,
};

pub fn compile_def(def: fun::syntax::declarations::Definition) -> core::syntax::Def {
    let mut initial_state: CompileState = CompileState {
        covars: context_covars(&def.context).into_iter().collect(),
    };
    let new_covar = initial_state.free_covar_from_state();
    let body = def.body.compile_with_cont(
        core::syntax::Covariable {
            covar: new_covar.clone(),
        }
        .into(),
        &mut initial_state,
    );

    let mut new_cont: Vec<(Covariable, ())> = context_covars(&def.context)
        .into_iter()
        .map(|cv| (cv, ()))
        .collect();
    new_cont.push((new_covar, ()));

    core::syntax::Def {
        name: def.name,
        pargs: context_vars(&def.context)
            .into_iter()
            .map(|var| (var, ()))
            .collect(),
        cargs: new_cont,
        body,
    }
}

pub fn compile_decl(decl: fun::syntax::declarations::Declaration) -> core::syntax::Def {
    match decl {
        fun::syntax::declarations::Declaration::Definition(d) => compile_def(d),
        fun::syntax::declarations::Declaration::DataDefinition(_) => {
            todo!("Not implemented in Core yet")
        }
        fun::syntax::declarations::Declaration::CodataDefinition(_) => {
            todo!("Not implemented in Core yet")
        }
    }
}

pub fn compile_prog(prog: fun::syntax::declarations::Module) -> core::syntax::Prog {
    core::syntax::Prog {
        prog_defs: prog.declarations.into_iter().map(compile_decl).collect(),
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::program::{compile_def, compile_prog};
    use fun::{
        syntax::context::ContextBinding,
        syntax::declarations::{Definition, Module},
        syntax::terms::Term,
        syntax::types::Ty,
    };
    use std::rc::Rc;

    fn example_def1() -> Definition {
        Definition {
            name: "main".to_owned(),
            context: vec![ContextBinding::TypedCovar {
                covar: "a".to_owned(),
                ty: Ty::Int(),
            }],
            body: Term::Lit(1),
            ret_ty: (),
        }
    }
    fn example_def2() -> Definition {
        Definition {
            name: "id".to_owned(),
            context: vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::Int(),
            }],
            body: Term::Var("x".to_owned()),
            ret_ty: (),
        }
    }

    fn example_prog1() -> Module {
        Module {
            declarations: vec![],
        }
    }

    fn example_prog2() -> Module {
        Module {
            declarations: vec![example_def1().into(), example_def2().into()],
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

        let def1 = result.prog_defs.get(0).unwrap();
        let def2 = result.prog_defs.get(1).unwrap();

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
