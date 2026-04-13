//! This module defines the translation of top-level functions.

use crate::{
    compile::{Compile, CompileState},
    context::compile_context,
    types::compile_ty,
};
use core_lang::syntax::{CodataDeclaration, names::Identifier};
use fun::{
    syntax::{names::Name, types::OptTyped},
    traits::used_binders::UsedBinders,
};

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

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
    let mut used_vars = def.context.vars();

    let mut context = compile_context(def.context);

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
        core_lang::syntax::terms::XVar::covar(Identifier::new(new_covar.clone()), ty).into(),
        &mut state,
    );

    context
        .bindings
        .push(core_lang::syntax::context::ContextBinding {
            var: Identifier::new(new_covar),
            chi: core_lang::syntax::context::Chirality::Cns,
            quantity: todo!(),
            ty: compile_ty(&def.ret_ty),
        });

    def_plus_lifted_statements.push_front(core_lang::syntax::Def {
        name: Identifier::new(def.name),
        context,
        body,
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
    let mut used_vars = def.context.vars();
    let context = compile_context(def.context);

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
            Identifier::new(new_var.clone()),
            core_lang::syntax::Statement::Exit(core_lang::syntax::statements::Exit {
                arg: Rc::new(
                    core_lang::syntax::terms::XVar::var(Identifier::new(new_var), ty.clone())
                        .into(),
                ),
                ty: ty.clone(),
            }),
            ty,
        )
        .into(),
        &mut state,
    );

    def_plus_lifted_statements.push_front(core_lang::syntax::Def {
        name: Identifier::new(def.name),
        context,
        body,
    });

    def_plus_lifted_statements
}
