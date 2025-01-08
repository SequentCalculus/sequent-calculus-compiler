//! Compiling a well-typed program from the source language `Fun` to the intermediate language `Core`.

use crate::definition::{CompileState, CompileWithCont};
use core_lang::syntax::context::Context;
use core_lang::syntax::declaration::CodataDeclaration;
use core_lang::syntax::term::Cns;
use core_lang::traits::*;
use fun::syntax::types::OptTyped;
use fun::traits::UsedBinders;

use std::collections::{HashSet, VecDeque};

pub fn compile_subst(
    subst: fun::syntax::substitution::Substitution,
    state: &mut CompileState,
) -> core_lang::syntax::substitution::Substitution {
    core_lang::syntax::substitution::Substitution(
        subst
            .into_iter()
            .map(|binding| match binding {
                fun::syntax::substitution::SubstitutionBinding::TermBinding(term) => {
                    let ty = compile_ty(
                        term.get_type()
                            .expect("Types should be annotated before translation"),
                    );
                    core_lang::syntax::substitution::SubstitutionBinding::ProducerBinding(
                        term.compile_opt(state, ty),
                    )
                }
                fun::syntax::substitution::SubstitutionBinding::CovarBinding { covar, ty } => {
                    core_lang::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                        core_lang::syntax::term::XVar {
                            prdcns: Cns,
                            var: covar,
                            ty: compile_ty(
                                ty.expect("Types should be annotated before translation"),
                            ),
                        }
                        .into(),
                    )
                }
            })
            .collect(),
    )
}

pub fn compile_ty(ty: fun::syntax::types::Ty) -> core_lang::syntax::types::Ty {
    match ty {
        fun::syntax::types::Ty::I64 { .. } => core_lang::syntax::types::Ty::I64,
        fun::syntax::types::Ty::Decl { name, .. } => core_lang::syntax::types::Ty::Decl(name),
    }
}

pub fn compile_context(
    ctx: fun::syntax::context::TypingContext,
) -> core_lang::syntax::context::TypingContext {
    Context {
        bindings: ctx
            .bindings
            .into_iter()
            .map(|binding| match binding {
                fun::syntax::context::ContextBinding::TypedVar { var, ty } => {
                    core_lang::syntax::context::ContextBinding::VarBinding {
                        var,
                        ty: compile_ty(ty),
                    }
                }
                fun::syntax::context::ContextBinding::TypedCovar { covar, ty } => {
                    core_lang::syntax::context::ContextBinding::CovarBinding {
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
) -> core_lang::syntax::Def {
    let mut new_context = compile_context(def.context);

    let mut used_vars = HashSet::new();
    def.body.used_binders(&mut used_vars);
    used_vars.extend(new_context.vars());
    used_vars.extend(new_context.covars());
    let mut state: CompileState = CompileState {
        used_vars,
        codata_types,
    };

    let new_covar = state.fresh_covar();
    let ty = compile_ty(
        def.body
            .get_type()
            .expect("Types should be annotated before translation"),
    );

    let body = def.body.compile_with_cont(
        core_lang::syntax::term::XVar {
            prdcns: Cns,
            var: new_covar.clone(),
            ty,
        }
        .into(),
        &mut state,
    );

    new_context
        .bindings
        .push(core_lang::syntax::context::ContextBinding::CovarBinding {
            covar: new_covar,
            ty: compile_ty(def.ret_ty),
        });

    core_lang::syntax::Def {
        name: def.name,
        context: new_context,
        body,
        used_vars: state.used_vars.clone(),
    }
}

pub fn compile_main(
    def: fun::syntax::declarations::Definition,
    codata_types: &'_ [CodataDeclaration],
) -> core_lang::syntax::Def {
    let new_context = compile_context(def.context);

    let mut used_vars = HashSet::new();
    def.body.used_binders(&mut used_vars);
    used_vars.extend(new_context.vars());
    used_vars.extend(new_context.covars());
    let mut state: CompileState = CompileState {
        used_vars,
        codata_types,
    };

    let new_var = state.fresh_var();
    let ty = compile_ty(
        def.body
            .get_type()
            .expect("Types should be annotated before translation"),
    );

    let body = def.body.compile_with_cont(
        core_lang::syntax::term::Mu::tilde_mu(
            &new_var,
            core_lang::syntax::Statement::Done(ty.clone()),
            ty,
        )
        .into(),
        &mut state,
    );

    core_lang::syntax::Def {
        name: def.name,
        context: new_context,
        body,
        used_vars: state.used_vars.clone(),
    }
}

pub fn compile_ctor(
    ctor: fun::syntax::declarations::CtorSig,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Data> {
    core_lang::syntax::declaration::XtorSig {
        xtor: core_lang::syntax::declaration::Data,
        name: ctor.name,
        args: compile_context(ctor.args),
    }
}

pub fn compile_dtor(
    dtor: fun::syntax::declarations::DtorSig,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Codata> {
    let mut new_args = compile_context(dtor.args);

    let new_cv = fresh_var(&mut new_args.covars().into_iter().collect(), "a");

    new_args
        .bindings
        .push(core_lang::syntax::context::ContextBinding::CovarBinding {
            covar: new_cv,
            ty: compile_ty(dtor.cont_ty),
        });
    core_lang::syntax::declaration::XtorSig {
        xtor: core_lang::syntax::declaration::Codata,
        name: dtor.name,
        args: new_args,
    }
}

pub fn compile_prog(prog: fun::syntax::declarations::Module) -> core_lang::syntax::Prog {
    let mut defs = Vec::new();
    let mut data_types = Vec::new();
    let mut codata_types = Vec::new();

    for declaration in prog.declarations {
        match declaration {
            fun::syntax::declarations::Declaration::Definition(definition) => defs.push(definition),
            fun::syntax::declarations::Declaration::DataDeclaration(data) => {
                data_types.push(core_lang::syntax::declaration::TypeDeclaration {
                    dat: core_lang::syntax::declaration::Data,
                    name: data.name,
                    xtors: data.ctors.into_iter().map(compile_ctor).collect(),
                })
            }
            fun::syntax::declarations::Declaration::CodataDeclaration(codata) => {
                codata_types.push(core_lang::syntax::declaration::TypeDeclaration {
                    dat: core_lang::syntax::declaration::Codata,
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

    core_lang::syntax::Prog {
        defs: defs_translated.into(),
        data_types,
        codata_types,
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::program::{compile_def, compile_prog};
    use codespan::Span;
    use core_lang::syntax::context::Context;
    use fun::syntax::{
        declarations::{Definition, Module},
        terms::{Lit, Var},
        types::Ty,
    };
    use std::collections::HashSet;

    fn example_def1() -> Definition {
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_covar("a", Ty::mk_i64());
        Definition {
            span: Span::default(),
            name: "main".to_owned(),
            context: ctx,
            body: Lit::mk(1).into(),
            ret_ty: Ty::mk_i64(),
        }
    }
    fn example_def2() -> Definition {
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        Definition {
            span: Span::default(),
            name: "id".to_owned(),
            context: ctx,
            body: Var {
                span: Span::default(),
                var: "x".to_owned(),
                ty: Some(Ty::mk_i64()),
            }
            .into(),
            ret_ty: Ty::mk_i64(),
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
        let mut ctx = Context::new();
        ctx.add_covar("a", core_lang::syntax::types::Ty::I64);
        ctx.add_covar("a0", core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::Def {
            name: "main".to_string(),
            context: ctx,
            body: core_lang::syntax::statement::Cut::new(
                core_lang::syntax::term::Literal::new(1),
                core_lang::syntax::term::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::types::Ty::I64,
            )
            .into(),
            used_vars: HashSet::from(["a".to_string(), "a0".to_string()]),
        };
        assert_eq!(result.name, expected.name);
        assert_eq!(result.context, expected.context);
        assert_eq!(result.body, expected.body);
    }
    #[test]
    fn compile_def2() {
        let result = compile_def(example_def2(), &[]);
        let mut ctx = Context::new();
        ctx.add_var("x", core_lang::syntax::types::Ty::I64);
        ctx.add_covar("a0", core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::Def {
            name: "id".to_owned(),
            context: ctx,
            body: core_lang::syntax::statement::Cut::new(
                core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::term::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::types::Ty::I64,
            )
            .into(),
            used_vars: HashSet::from(["x".to_string(), "a0".to_string()]),
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
        let mut ctx = Context::new();
        ctx.add_covar("a", core_lang::syntax::types::Ty::I64);
        let expected1 = core_lang::syntax::Def {
            name: "main".to_owned(),
            context: ctx,
            body: core_lang::syntax::statement::Cut::new(
                core_lang::syntax::term::Literal::new(1),
                core_lang::syntax::term::Mu::tilde_mu(
                    "x0",
                    core_lang::syntax::Statement::Done(core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::types::Ty::I64,
            )
            .into(),
            used_vars: HashSet::from(["a".to_string(), "x0".to_string()]),
        };
        let mut ctx = Context::new();
        ctx.add_var("x", core_lang::syntax::types::Ty::I64);
        ctx.add_covar("a0", core_lang::syntax::types::Ty::I64);
        let expected2 = core_lang::syntax::Def {
            name: "id".to_owned(),
            context: ctx,
            body: core_lang::syntax::statement::Cut::new(
                core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::term::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::types::Ty::I64,
            )
            .into(),
            used_vars: HashSet::from(["x".to_string(), "a0".to_string()]),
        };

        let def1 = result.defs.get(0).unwrap();
        let def2 = result.defs.get(1).unwrap();

        assert_eq!(def1, &expected1);
        assert_eq!(def2, &expected2);
    }
}
