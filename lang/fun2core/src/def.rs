//! This module defines the translation of top-level functions.

use crate::{
    compile::{Compile, CompileState},
    context::compile_context,
    types::compile_ty,
};
use core_lang::syntax::names::Identifier;
use core_lang::syntax::type_params::{ParamPolarity, TypeParam};
use core_lang::syntax::{CodataDeclaration, DataDeclaration};
use fun::syntax::names::Name;
use fun::traits::{OptTyped, UsedBinders};

use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

/// This function translates a [top-level function in Fun](fun::syntax::declarations::Def) to a
/// [top-level function in Core](core_lang::syntax::Def).
/// - `def` is the top-level function to translate.
/// - `codata_types` is the list of codata types in the corresponding [Fun](fun) program.
/// - `data_types` is the list of data types in the corresponding [Fun](fun) program.
/// - `used_labels` is the set of labels of top-level functions in the corresponding [Fun](fun)
///   program.
/// - `max_id` is the maximum identifier used so far, needed for generating fresh identifiers.
/// - `type_params_subst` maps this def's ambient type parameter names (its own plus any enclosing
///   declaration's) to their fresh Core identifier and declared polarity.
/// - `type_params` is this def's own type parameters, already translated to Core.
///
/// # Panics
///
/// A panic is caused if the types are not annotated in the program.
pub fn compile_def(
    def: fun::syntax::declarations::Def,
    codata_types: &[CodataDeclaration],
    data_types: &[DataDeclaration],
    used_labels: &mut HashSet<Name>,
    max_id: &mut usize,
    type_params_subst: Rc<HashMap<String, (Identifier, ParamPolarity)>>,
    type_params: Vec<TypeParam>,
) -> VecDeque<core_lang::syntax::Def> {
    let mut used_vars = def.context.vars();

    let mut context = compile_context(def.context, type_params_subst.clone());

    def.body.used_binders(&mut used_vars);
    // we sometimes create new top-level labels during the translation, so we need to collect them
    let mut def_plus_lifted_statements = VecDeque::new();
    let mut state: CompileState = CompileState {
        used_vars,
        codata_types,
        data_types,
        used_labels,
        current_label: &def.name,
        lifted_statements: &mut def_plus_lifted_statements,
        max_id,
    };

    let new_covar = state.fresh_covar();
    let ty = compile_ty(
        &def.body
            .get_type()
            .expect("Types should be annotated before translation"),
        type_params_subst.clone(),
    );

    let body = def.body.compile_with_cont(
        core_lang::syntax::terms::XVar::covar(Identifier::new(new_covar.clone()), ty).into(),
        &mut state,
        type_params_subst.clone(),
    );

    context
        .bindings
        .push(core_lang::syntax::context::ContextBinding {
            var: Identifier::new(new_covar),
            chi: core_lang::syntax::context::Chirality::Cns,
            ty: compile_ty(&def.ret_ty, type_params_subst),
        });

    def_plus_lifted_statements.push_front(core_lang::syntax::Def {
        name: Identifier::new(def.name),
        type_params,
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
/// - `data_types` is the list of data types in the corresponding [Fun](fun) program.
/// - `used_labels` is the set of labels of top-level functions in the corresponding [Fun](fun)
///   program.
/// - `max_id` is the maximum identifier used so far, needed for generating fresh identifiers.
/// - `type_params_subst` maps every top-level declaration's type parameter names to their fresh
///   Core identifier and declared polarity (`main` has none of its own).
///
/// # Panics
///
/// A panic is caused if the types are not annotated in the program
pub fn compile_main(
    def: fun::syntax::declarations::Def,
    codata_types: &[CodataDeclaration],
    data_types: &[DataDeclaration],
    used_labels: &mut HashSet<Name>,
    max_id: &mut usize,
    type_params_subst: Rc<HashMap<String, (Identifier, ParamPolarity)>>,
) -> VecDeque<core_lang::syntax::Def> {
    let mut used_vars = def.context.vars();
    let context = compile_context(def.context, type_params_subst.clone());

    def.body.used_binders(&mut used_vars);
    // we sometimes create new top-level labels during the translation, so we need to collect them
    let mut def_plus_lifted_statements = VecDeque::new();
    let mut state: CompileState = CompileState {
        used_vars,
        codata_types,
        data_types,
        used_labels,
        current_label: &def.name,
        lifted_statements: &mut def_plus_lifted_statements,
        max_id,
    };

    let new_var = state.fresh_var();
    let ty = compile_ty(
        &def.body
            .get_type()
            .expect("Types should be annotated before translation"),
        type_params_subst.clone(),
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
        type_params_subst,
    );

    def_plus_lifted_statements.push_front(core_lang::syntax::Def {
        name: Identifier::new(def.name),
        type_params: vec![],
        context,
        body,
    });

    def_plus_lifted_statements
}
