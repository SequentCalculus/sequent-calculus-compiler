//! Compilation for [fun::syntax::declarations::Def]
//! Compiles to [core_lang::syntax::Def]
use crate::{
    compile::{CompileState, CompileWithCont},
    context::compile_context,
    types::compile_ty,
};
use core_lang::syntax::CodataDeclaration;
use fun::{
    syntax::{Name, types::OptTyped},
    traits::used_binders::UsedBinders,
};

use std::collections::{HashSet, VecDeque};

/// Compiles [fun::syntax::declarations::Def] to [core_lang::syntax::Def]
pub fn compile_def(
    def: fun::syntax::declarations::Def,
    codata_types: &'_ [CodataDeclaration],
    used_labels: &mut HashSet<Name>,
) -> VecDeque<core_lang::syntax::Def> {
    let mut context = compile_context(def.context);

    let mut used_vars = context.vars();
    def.body.used_binders(&mut used_vars);
    let mut def_plus_lifted_statements = VecDeque::new();
    let mut state: CompileState = CompileState {
        used_vars,
        codata_types,
        used_labels,
        current_label: &def.name,
        lifted_statements: &mut def_plus_lifted_statements,
    };

    let new_covar = state.fresh_covar();
    let ty = compile_ty(
        &def.body
            .get_type()
            .expect("Types should be annotated before translation"),
    );

    let body = def.body.compile_with_cont(
        core_lang::syntax::terms::XVar::covar(&new_covar, ty).into(),
        &mut state,
    );

    context
        .bindings
        .push(core_lang::syntax::context::ContextBinding {
            var: new_covar,
            chi: core_lang::syntax::context::Chirality::Cns,
            ty: compile_ty(&def.ret_ty),
        });

    let used_vars = state.used_vars;
    def_plus_lifted_statements.push_front(core_lang::syntax::Def {
        name: def.name,
        context,
        body,
        used_vars,
    });

    def_plus_lifted_statements
}

/// Compiles the main [Definition][fun::syntax::declarations::Def] to [core_lang]
pub fn compile_main(
    def: fun::syntax::declarations::Def,
    codata_types: &'_ [CodataDeclaration],
    used_labels: &mut HashSet<Name>,
) -> VecDeque<core_lang::syntax::Def> {
    let context = compile_context(def.context);

    let mut used_vars = context.vars();
    def.body.used_binders(&mut used_vars);
    let mut def_plus_lifted_statements = VecDeque::new();
    let mut state: CompileState = CompileState {
        used_vars,
        codata_types,
        used_labels,
        current_label: &def.name,
        lifted_statements: &mut def_plus_lifted_statements,
    };

    let new_var = state.fresh_var();
    let ty = compile_ty(
        &def.body
            .get_type()
            .expect("Types should be annotated before translation"),
    );

    let body = def.body.compile_with_cont(
        core_lang::syntax::terms::Mu::tilde_mu(
            &new_var,
            core_lang::syntax::Statement::Exit(core_lang::syntax::statements::Exit::exit(
                core_lang::syntax::terms::XVar::var(&new_var, ty.clone()),
                ty.clone(),
            )),
            ty,
        )
        .into(),
        &mut state,
    );

    let used_vars = state.used_vars;
    def_plus_lifted_statements.push_front(core_lang::syntax::Def {
        name: def.name,
        context,
        body,
        used_vars,
    });

    def_plus_lifted_statements
}
