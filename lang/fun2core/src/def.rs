//! This module defines the translation of top-level functions.

use crate::{
    compile::{Compile, CompileState},
    context::compile_context,
    types::compile_ty,
};
use core_lang::syntax::CodataDeclaration;
use fun::{
    syntax::{Name, types::OptTyped},
    traits::used_binders::UsedBinders,
};

use std::collections::{HashSet, VecDeque};

/// This function translates a [top-level function in Fun](fun::syntax::declarations::Def) to a
/// [top-level function in Core](core_lang::syntax::Def).
/// - `def` is the top-level function to translate.
/// - `codata_types` is the list of codata types in the corresponding [Fun](fun) program.
/// - `used_labels` is the set of labels of top-level functions in the corresponding [Fun](fun)
///   program.
///
/// # Panics
///
/// A panic is caused if the types are not annotated in the program.
pub fn compile_def(
    def: fun::syntax::declarations::Def,
    codata_types: &'_ [CodataDeclaration],
    used_labels: &mut HashSet<Name>,
) -> VecDeque<core_lang::syntax::Def> {
    let mut context = compile_context(def.context);

    let mut used_vars = context.vars();
    def.body.used_binders(&mut used_vars);
    // we sometimes create new top-level labels during the translation, so we need to collect them
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
/// This function translates the top-level function `main` in [Fun](fun) to [Core](core_lang). In
/// contrast to other top-level functions, it does not obtain an additional consumer parameter, but
/// instead its body is translated with a consumer that terminates the program.
/// - `def` is the top-level function `main`.
/// - `codata_types` is the list of codata types in the corresponding [Fun](fun) program.
/// - `used_labels` is the set of labels of top-level functions in the corresponding [Fun](fun)
///   program.
///
/// # Panics
///
/// A panic is caused if the types are not annotated in the program.
pub fn compile_main(
    def: fun::syntax::declarations::Def,
    codata_types: &'_ [CodataDeclaration],
    used_labels: &mut HashSet<Name>,
) -> VecDeque<core_lang::syntax::Def> {
    let context = compile_context(def.context);

    let mut used_vars = context.vars();
    def.body.used_binders(&mut used_vars);
    // we sometimes create new top-level labels during the translation, so we need to collect them
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
