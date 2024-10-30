//! Compiling a program from the source language `Fun` to the intermediate language `Core`.

use crate::definition::{CompileState, CompileWithCont};
use core::syntax::context::context_covars;
use core::syntax::term::Cns;
use core::traits::free_vars::fresh_covar;

pub fn compile_subst(
    subst: fun::syntax::substitution::Substitution,
    st: &mut CompileState,
) -> core::syntax::substitution::Substitution {
    subst
        .into_iter()
        .map(|bnd| match bnd {
            fun::syntax::substitution::SubstitutionBinding::TermBinding {
                term: t,
                ty: Some(ty),
            } => core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                prd: t.compile_opt(st),
                ty: compile_ty(ty),
            },
            fun::syntax::substitution::SubstitutionBinding::CovarBinding {
                covar: cv,
                ty: Some(ty),
            } => core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                cns: core::syntax::term::XVar {
                    prdcns: Cns,
                    var: cv,
                }
                .into(),
                ty: compile_ty(ty),
            },
            _ => panic!("Substitutions should always have annotated types"),
        })
        .collect()
}
pub fn compile_ty(ty: fun::syntax::types::Ty) -> core::syntax::types::Ty {
    match ty {
        fun::syntax::types::Ty::Int { .. } => core::syntax::types::Ty::Int(),
        fun::syntax::types::Ty::Decl { name, .. } => core::syntax::types::Ty::Decl(name),
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

pub fn compile_def(
    def: fun::syntax::declarations::Definition,
    st: &mut CompileState,
) -> core::syntax::Def {
    let mut new_context = compile_context(def.context);

    st.covars = context_covars(&new_context).into_iter().collect();
    let new_covar = st.free_covar_from_state();
    let body = def.body.compile_with_cont(
        core::syntax::term::XVar {
            prdcns: Cns,
            var: new_covar.clone(),
        }
        .into(),
        st,
    );

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

pub fn compile_ctor(
    ctor: fun::syntax::declarations::CtorSig,
) -> core::syntax::declaration::XtorSig<core::syntax::declaration::Data> {
    core::syntax::declaration::XtorSig {
        xtor: core::syntax::declaration::Data,
        name: ctor.name,
        args: compile_context(ctor.args),
    }
}
pub fn compile_dtor(
    dtor: fun::syntax::declarations::DtorSig,
) -> core::syntax::declaration::XtorSig<core::syntax::declaration::Codata> {
    let mut new_args = compile_context(dtor.args);

    let new_cv = fresh_covar(&context_covars(&new_args).into_iter().collect());

    new_args.push(core::syntax::context::ContextBinding::CovarBinding {
        covar: new_cv,
        ty: compile_ty(dtor.cont_ty),
    });
    core::syntax::declaration::XtorSig {
        xtor: core::syntax::declaration::Codata,
        name: dtor.name,
        args: new_args,
    }
}

pub fn compile_decl(
    decl: fun::syntax::declarations::Declaration,
    st: &mut CompileState,
) -> core::syntax::program::Declaration {
    match decl {
        fun::syntax::declarations::Declaration::Definition(d) => compile_def(d, st).into(),
        fun::syntax::declarations::Declaration::DataDeclaration(data) => {
            let new_decl = core::syntax::declaration::TypeDeclaration {
                dat: core::syntax::declaration::Data,
                name: data.name.clone(),
                xtors: data.ctors.into_iter().map(compile_ctor).collect(),
            };
            st.data_decls.push(new_decl.clone());
            new_decl.into()
        }
        fun::syntax::declarations::Declaration::CodataDeclaration(codata) => {
            let new_decl = core::syntax::declaration::TypeDeclaration {
                dat: core::syntax::declaration::Codata,
                name: codata.name.clone(),
                xtors: codata.dtors.into_iter().map(compile_dtor).collect(),
            };
            st.codata_decls.push(new_decl.clone());
            new_decl.into()
        }
    }
}

pub fn compile_prog(prog: fun::syntax::declarations::Module) -> core::syntax::Prog {
    let mut st = CompileState::default();
    core::syntax::Prog {
        prog_decls: prog
            .declarations
            .into_iter()
            .map(|decl| compile_decl(decl, &mut st))
            .collect(),
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::program::{compile_def, compile_prog, CompileState};
    use codespan::Span;
    use core::syntax::term::{Cns, Prd};
    use fun::syntax::{
        context::ContextBinding,
        declarations::{Definition, Module},
        terms::{Lit, Var},
        types::Ty,
    };
    use std::rc::Rc;

    fn example_def1() -> Definition {
        Definition {
            span: Span::default(),
            name: "main".to_owned(),
            context: vec![ContextBinding::TypedCovar {
                covar: "a".to_owned(),
                ty: Ty::mk_int(),
            }],
            body: Lit::mk(1).into(),
            ret_ty: Ty::mk_int(),
        }
    }
    fn example_def2() -> Definition {
        Definition {
            span: Span::default(),
            name: "id".to_owned(),
            context: vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            body: Var::mk("x").into(),
            ret_ty: Ty::mk_int(),
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
        let result = compile_def(example_def1(), &mut CompileState::default());
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
            body: core::syntax::statement::Cut {
                producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                ty: core::syntax::types::Ty::Int(),
                consumer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Cns,
                        var: "a0".to_owned(),
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
        let result = compile_def(example_def2(), &mut CompileState::default());
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
            body: core::syntax::statement::Cut {
                producer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                ty: core::syntax::types::Ty::Int(),
                consumer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Cns,
                        var: "a0".to_owned(),
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
        assert!(result.prog_decls.is_empty())
    }

    #[test]
    fn compile_prog2() {
        let result = compile_prog(example_prog2());
        assert_eq!(result.prog_decls.len(), 2);
        let expected1: core::syntax::program::Declaration = core::syntax::Def {
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
            body: core::syntax::statement::Cut {
                producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                ty: core::syntax::types::Ty::Int(),
                consumer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Cns,
                        var: "a0".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        }
        .into();
        let expected2: core::syntax::program::Declaration = core::syntax::Def {
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
            body: core::syntax::statement::Cut {
                producer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                ty: core::syntax::types::Ty::Int(),
                consumer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Cns,
                        var: "a0".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        }
        .into();

        let def1 = result.prog_decls.get(0).unwrap();
        let def2 = result.prog_decls.get(1).unwrap();

        assert_eq!(def1, &expected1);
        assert_eq!(def2, &expected2);
    }
}
