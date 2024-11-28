//! Compiling a program from the source language `Fun` to the intermediate language `Core`.

use crate::definition::{CompileState, CompileWithCont};
use core::syntax::context::Context;
use core::syntax::declaration::CodataDeclaration;
use core::syntax::term::Cns;
use core::traits::free_vars::fresh_var;
use fun::syntax::types::OptTyped;

use std::collections::VecDeque;

pub fn compile_subst(
    subst: fun::syntax::substitution::Substitution,
    st: &mut CompileState,
) -> core::syntax::substitution::Substitution {
    subst
        .into_iter()
        .map(|bnd| match bnd {
            fun::syntax::substitution::SubstitutionBinding::TermBinding(t) => {
                let ty = compile_ty(
                    t.get_type()
                        .expect("Types should be annotated before translation"),
                );
                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                    t.compile_opt(st, ty),
                )
            }
            fun::syntax::substitution::SubstitutionBinding::CovarBinding { covar: cv, ty } => {
                core::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                    core::syntax::term::XVar {
                        prdcns: Cns,
                        var: cv,
                        ty: compile_ty(ty.expect("Types should be annotated before translation")),
                    }
                    .into(),
                )
            }
        })
        .collect()
}
pub fn compile_ty(ty: fun::syntax::types::Ty) -> core::syntax::types::Ty {
    match ty {
        fun::syntax::types::Ty::Int { .. } => core::syntax::types::Ty::Int,
        fun::syntax::types::Ty::Decl { name, .. } => core::syntax::types::Ty::Decl(name),
    }
}

pub fn compile_context(
    ctx: fun::syntax::context::TypingContext,
) -> core::syntax::context::TypingContext {
    Context {
        bindings: ctx
            .bindings
            .into_iter()
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
            .collect(),
    }
}

pub fn compile_def(
    def: fun::syntax::declarations::Definition,
    codata_types: &'_ [CodataDeclaration],
) -> core::syntax::Def {
    let mut new_context = compile_context(def.context);

    let mut used_vars = new_context.vars();
    used_vars.extend(new_context.covars());
    let mut initial_state: CompileState = CompileState {
        covars: used_vars,
        codata_types,
    };
    let new_covar = initial_state.fresh_covar();
    let ty = compile_ty(
        def.body
            .get_type()
            .expect("Types should be annotated before translation"),
    );
    let body = def.body.compile_with_cont(
        core::syntax::term::XVar {
            prdcns: Cns,
            var: new_covar.clone(),
            ty,
        }
        .into(),
        &mut initial_state,
    );

    new_context
        .bindings
        .push(core::syntax::context::ContextBinding::CovarBinding {
            covar: new_covar,
            ty: compile_ty(def.ret_ty),
        });

    core::syntax::Def {
        name: def.name,
        context: new_context,
        body,
    }
}

pub fn compile_main(
    def: fun::syntax::declarations::Definition,
    codata_types: &'_ [CodataDeclaration],
) -> core::syntax::Def {
    let new_context = compile_context(def.context);

    let mut used_vars = new_context.vars();
    used_vars.extend(new_context.covars());
    let mut initial_state: CompileState = CompileState {
        covars: used_vars,
        codata_types,
    };
    let new_var = fresh_var(&mut new_context.vars(), "x");
    let ty = compile_ty(
        def.body
            .get_type()
            .expect("Types should be annotated before translation"),
    );
    let body = def.body.compile_with_cont(
        core::syntax::term::Mu::tilde_mu(&new_var, core::syntax::Statement::Done(ty.clone()), ty)
            .into(),
        &mut initial_state,
    );

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

    let new_cv = fresh_var(&mut new_args.covars().into_iter().collect(), "a");

    new_args
        .bindings
        .push(core::syntax::context::ContextBinding::CovarBinding {
            covar: new_cv,
            ty: compile_ty(dtor.cont_ty),
        });
    core::syntax::declaration::XtorSig {
        xtor: core::syntax::declaration::Codata,
        name: dtor.name,
        args: new_args,
    }
}

pub fn compile_prog(prog: fun::syntax::declarations::Module) -> core::syntax::Prog {
    let mut defs = Vec::new();
    let mut data_types = Vec::new();
    let mut codata_types = Vec::new();

    for declaration in prog.declarations {
        match declaration {
            fun::syntax::declarations::Declaration::Definition(definition) => defs.push(definition),
            fun::syntax::declarations::Declaration::DataDeclaration(data) => {
                data_types.push(core::syntax::declaration::TypeDeclaration {
                    dat: core::syntax::declaration::Data,
                    name: data.name,
                    xtors: data.ctors.into_iter().map(compile_ctor).collect(),
                })
            }
            fun::syntax::declarations::Declaration::CodataDeclaration(codata) => {
                codata_types.push(core::syntax::declaration::TypeDeclaration {
                    dat: core::syntax::declaration::Codata,
                    name: codata.name,
                    xtors: codata.dtors.into_iter().map(compile_dtor).collect(),
                })
            }
        }
    }

    let mut defs_translated = VecDeque::new();
    for def in defs {
        if def.name == "main" {
            defs_translated.push_front(compile_main(def, codata_types.as_slice()))
        } else {
            defs_translated.push_back(compile_def(def, codata_types.as_slice()))
        }
    }

    core::syntax::Prog {
        defs: defs_translated.into(),
        data_types,
        codata_types,
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::program::{compile_def, compile_prog};
    use codespan::Span;
    use core::syntax::{
        context::Context,
        term::{Cns, Prd},
    };
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
            context: fun::syntax::context::TypingContext {
                span: Span::default(),
                bindings: vec![ContextBinding::TypedCovar {
                    covar: "a".to_owned(),
                    ty: Ty::mk_int(),
                }],
            },
            body: Lit::mk(1).into(),
            ret_ty: Ty::mk_int(),
        }
    }
    fn example_def2() -> Definition {
        Definition {
            span: Span::default(),
            name: "id".to_owned(),
            context: fun::syntax::context::TypingContext {
                span: Span::default(),
                bindings: vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                }],
            },
            body: Var {
                span: Span::default(),
                var: "x".to_owned(),
                ty: Some(Ty::mk_int()),
            }
            .into(),
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
        let result = compile_def(example_def1(), &[]);
        let expected = core::syntax::Def {
            name: "main".to_owned(),
            context: Context {
                bindings: vec![
                    core::syntax::context::ContextBinding::CovarBinding {
                        covar: "a".to_owned(),
                        ty: core::syntax::types::Ty::Int,
                    },
                    core::syntax::context::ContextBinding::CovarBinding {
                        covar: "a0".to_owned(),
                        ty: core::syntax::types::Ty::Int,
                    },
                ],
            },
            body: core::syntax::statement::Cut {
                producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                ty: core::syntax::types::Ty::Int,
                consumer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Cns,
                        var: "a0".to_owned(),
                        ty: core::syntax::types::Ty::Int,
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
        let result = compile_def(example_def2(), &[]);
        let expected = core::syntax::Def {
            name: "id".to_owned(),
            context: Context {
                bindings: vec![
                    core::syntax::context::ContextBinding::VarBinding {
                        var: "x".to_owned(),
                        ty: core::syntax::types::Ty::Int,
                    },
                    core::syntax::context::ContextBinding::CovarBinding {
                        covar: "a0".to_owned(),
                        ty: core::syntax::types::Ty::Int,
                    },
                ],
            },
            body: core::syntax::statement::Cut {
                producer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                        ty: core::syntax::types::Ty::Int,
                    }
                    .into(),
                ),
                ty: core::syntax::types::Ty::Int,

                consumer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Cns,
                        var: "a0".to_owned(),
                        ty: core::syntax::types::Ty::Int,
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
        assert!(result.defs.is_empty());
        assert!(result.data_types.is_empty());
        assert!(result.codata_types.is_empty());
    }

    #[test]
    fn compile_prog2() {
        let result = compile_prog(example_prog2());
        assert_eq!(result.defs.len(), 2);
        let expected1 = core::syntax::Def {
            name: "main".to_owned(),
            context: Context {
                bindings: vec![core::syntax::context::ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: core::syntax::types::Ty::Int,
                }],
            },
            body: core::syntax::statement::Cut {
                producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                ty: core::syntax::types::Ty::Int,

                consumer: Rc::new(
                    core::syntax::term::Mu::tilde_mu(
                        "x0",
                        core::syntax::Statement::Done(core::syntax::types::Ty::Int),
                        core::syntax::types::Ty::Int,
                    )
                    .into(),
                ),
            }
            .into(),
        };
        let expected2 = core::syntax::Def {
            name: "id".to_owned(),
            context: Context {
                bindings: vec![
                    core::syntax::context::ContextBinding::VarBinding {
                        var: "x".to_owned(),
                        ty: core::syntax::types::Ty::Int,
                    },
                    core::syntax::context::ContextBinding::CovarBinding {
                        covar: "a0".to_owned(),
                        ty: core::syntax::types::Ty::Int,
                    },
                ],
            },
            body: core::syntax::statement::Cut {
                producer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                        ty: core::syntax::types::Ty::Int,
                    }
                    .into(),
                ),
                ty: core::syntax::types::Ty::Int,

                consumer: Rc::new(
                    core::syntax::term::XVar {
                        prdcns: Cns,
                        var: "a0".to_owned(),
                        ty: core::syntax::types::Ty::Int,
                    }
                    .into(),
                ),
            }
            .into(),
        };

        let def1 = result.defs.get(0).unwrap();
        let def2 = result.defs.get(1).unwrap();

        assert_eq!(def1, &expected1);
        assert_eq!(def2, &expected2);
    }
}
