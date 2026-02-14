//! This module defines the translation for a clause in a pattern or copattern match.

use crate::{
    compile::{Compile, CompileState},
    context::compile_context,
    types::compile_ty,
};
use core_lang::syntax::{
    Chirality, ContextBinding, Statement,
    names::Ident,
    terms::{Cns, Prd},
};
use fun::syntax::types::OptTyped;

use std::rc::Rc;

/// This function translates a [clause of a pattern match in Fun](fun::syntax::terms::Clause) to a
/// [clause of a pattern match in Core](core_lang::syntax::terms::Clause).
pub fn compile_clause(
    clause: fun::syntax::terms::Clause,
    cont: core_lang::syntax::terms::Term<Cns>,
    state: &mut CompileState,
) -> core_lang::syntax::terms::Clause<Cns, Statement> {
    core_lang::syntax::terms::Clause {
        prdcns: Cns,
        xtor: Ident::new_with_zero(&clause.xtor),
        context: compile_context(clause.context),
        body: Rc::new(clause.body.compile_with_cont(cont, state)),
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
) -> core_lang::syntax::terms::Clause<Prd, Statement> {
    let ty = compile_ty(
        &clause
            .get_type()
            .expect("Types should be annotated before translation"),
    );
    let mut new_context = compile_context(clause.context);
    let new_covar = state.fresh_covar();
    new_context.bindings.push(ContextBinding {
        var: Ident::new_with_zero(&new_covar),
        chi: Chirality::Cns,
        ty: ty.clone(),
    });

    core_lang::syntax::terms::Clause {
        prdcns: Prd,
        xtor: Ident::new_with_zero(&clause.xtor),
        context: new_context,
        body: Rc::new(
            clause.body.compile_with_cont(
                core_lang::syntax::terms::XVar {
                    prdcns: Cns,
                    var: Ident::new_with_zero(&new_covar),
                    ty,
                }
                .into(),
                state,
            ),
        ),
    }
}
