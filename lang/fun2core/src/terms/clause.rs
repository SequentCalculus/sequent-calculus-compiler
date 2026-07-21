//! This module defines the translation for a clause in a pattern or copattern match.

use crate::{
    compile::{Compile, CompileState},
    context::compile_context,
    program::build_type_param_subst,
    types::{compile_ty, compile_type_params},
};
use core_lang::syntax::{
    Chirality, ContextBinding, Statement,
    names::Identifier,
    terms::{Cns, Prd},
};
use fun::traits::OptTyped;

use std::{collections::HashMap, rc::Rc};

/// This function translates a [clause of a pattern match in Fun](fun::syntax::terms::Clause) to a
/// [clause of a pattern match in Core](core_lang::syntax::terms::Clause).
pub fn compile_clause(
    clause: fun::syntax::terms::Clause,
    cont: core_lang::syntax::terms::Term<Cns>,
    state: &mut CompileState,
    type_params: Rc<HashMap<String, Identifier>>,
) -> core_lang::syntax::terms::Clause<Cns, Statement> {
    let clause_type_params = compile_type_params(&clause.type_params, state.max_id);
    let type_params_subst: Rc<HashMap<String, Identifier>> = Rc::new(
        (*type_params)
            .clone()
            .into_iter()
            .chain(build_type_param_subst(
                &clause.type_params.bindings,
                &clause_type_params,
            ))
            .collect(),
    );

    // lookup the concret name of the constructor in the data types
    let Some(xtor) = state.data_types.iter().find_map(|data_decl| {
        data_decl
            .xtors
            .iter()
            .find(|ctor| ctor.name.name == clause.xtor)
            .map(|ctor| ctor.name.clone())
    }) else {
        panic!("Constructor {} not found in data types", clause.xtor);
    };

    core_lang::syntax::terms::Clause {
        prdcns: Cns,
        xtor,
        type_params: clause_type_params,
        context: compile_context(clause.context, type_params_subst.clone()),
        body: Rc::new(
            clause
                .body
                .compile_with_cont(cont, state, type_params_subst),
        ),
    }
}

/// This function translates a [clause of a copattern match in Fun](fun::syntax::terms::Clause) to
/// a [clause of a copattern match in Core](core_lang::syntax::terms::Clause).
///
/// # Panics
///
/// A panic is caused if the types are not annotated in the program.
pub fn compile_coclause(
    clause: fun::syntax::terms::Clause,
    state: &mut CompileState,
    type_params: Rc<HashMap<String, Identifier>>,
) -> core_lang::syntax::terms::Clause<Prd, Statement> {
    let coclause_type_params = compile_type_params(&clause.type_params, state.max_id);
    let type_params_subst: Rc<HashMap<String, Identifier>> = Rc::new(
        (*type_params)
            .clone()
            .into_iter()
            .chain(build_type_param_subst(
                &clause.type_params.bindings,
                &coclause_type_params,
            ))
            .collect(),
    );

    let ty = compile_ty(
        &clause
            .get_type()
            .expect("Types should be annotated before translation"),
        type_params_subst.clone(),
    );
    let mut new_context = compile_context(clause.context, type_params_subst.clone());
    let new_covar = state.fresh_covar();
    new_context.bindings.push(ContextBinding {
        var: Identifier::new(new_covar.clone()),
        chi: Chirality::Cns,
        ty: ty.clone(),
    });

    // lookup the concret name of the destructor in the codata types
    let Some(xtor) = state.codata_types.iter().find_map(|codata_decl| {
        codata_decl
            .xtors
            .iter()
            .find(|dtor| dtor.name.name == clause.xtor)
            .map(|dtor| dtor.name.clone())
    }) else {
        panic!("Destructor {} not found in codata types", clause.xtor);
    };

    core_lang::syntax::terms::Clause {
        prdcns: Prd,
        xtor,
        type_params: coclause_type_params,
        context: new_context,
        body: Rc::new(
            clause.body.compile_with_cont(
                core_lang::syntax::terms::XVar {
                    prdcns: Cns,
                    var: Identifier::new(new_covar),
                    ty,
                }
                .into(),
                state,
                type_params_subst,
            ),
        ),
    }
}
