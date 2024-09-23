//! Compiling a program from the source language `Fun` to the intermediate language `Core`.

use crate::definition::{CompileState, CompileWithCont};
use fun::syntax::context::context_covars;

pub fn compile_ty(ty: fun::syntax::types::Ty) -> core::syntax::types::Ty {
    match ty {
        fun::syntax::types::Ty::Int() => core::syntax::types::Ty::Int(),
        fun::syntax::types::Ty::Decl(name) => core::syntax::types::Ty::Decl(name),
    }
}

pub fn compile_context(
    ctx: fun::syntax::context::TypingContext,
) -> core::syntax::context::TypingContext {
    ctx.into_iter()
        .map(|bnd| match bnd {
            fun::syntax::context::ContextBinding::TypedVar { var, ty } => {
                core::syntax::context::ContextBinding::VarBinding {
                    var,
                    ty: compile_ty(ty),
                }
            }
            fun::syntax::context::ContextBinding::TypedCovar { covar, ty } => {
                core::syntax::context::ContextBinding::CovarBinding {
                    covar,
                    ty: compile_ty(ty),
                }
            }
        })
        .collect()
}

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

    let mut new_context = compile_context(def.context);
    new_context.push(core::syntax::context::ContextBinding::CovarBinding {
        covar: new_covar,
        ty: compile_ty(def.ret_ty),
    });

    core::syntax::Def {
        name: def.name,
        context: new_context,
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
    use fun::syntax::{
        context::ContextBinding,
        declarations::{Definition, Module},
        terms::{Lit, Var},
        types::Ty,
    };
    use std::rc::Rc;

    fn example_def1() -> Definition {
        Definition {
            name: "main".to_owned(),
            context: vec![ContextBinding::TypedCovar {
                covar: "a".to_owned(),
                ty: Ty::Int(),
            }],
            body: Lit { val: 1 }.into(),
            ret_ty: Ty::Int(),
        }
    }
    fn example_def2() -> Definition {
        Definition {
            name: "id".to_owned(),
            context: vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::Int(),
            }],
            body: Var {
                var: "x".to_owned(),
            }
            .into(),
            ret_ty: Ty::Int(),
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
            context: vec![
                core::syntax::context::ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                },
                core::syntax::context::ContextBinding::CovarBinding {
                    covar: "a0".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                },
            ],
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
        assert_eq!(result.context, expected.context);
        assert_eq!(result.body, expected.body);
    }
    #[test]
    fn compile_def2() {
        let result = compile_def(example_def2());
        let expected = core::syntax::Def {
            name: "id".to_owned(),
            context: vec![
                core::syntax::context::ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                },
                core::syntax::context::ContextBinding::CovarBinding {
                    covar: "a0".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                },
            ],
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
        assert_eq!(result.context, expected.context);
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
            context: vec![
                core::syntax::context::ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                },
                core::syntax::context::ContextBinding::CovarBinding {
                    covar: "a0".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                },
            ],
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
            context: vec![
                core::syntax::context::ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                },
                core::syntax::context::ContextBinding::CovarBinding {
                    covar: "a0".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                },
            ],
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
        assert_eq!(def1.context, expected1.context);
        assert_eq!(def1.body, expected1.body);
        assert_eq!(def2.name, expected2.name);
        assert_eq!(def2.context, expected2.context);
        assert_eq!(def2.body, expected2.body);
    }
}
