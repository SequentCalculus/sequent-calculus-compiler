use crate::{
    compile::{CompileState, CompileWithCont},
    program::{compile_context, compile_ty},
};
use core_lang::syntax::{
    terms::{Cns, Prd},
    Chirality, ContextBinding, Statement,
};
use fun::syntax::types::OptTyped;

use std::rc::Rc;

pub fn compile_clause(
    clause: fun::syntax::terms::Clause,
    cont: core_lang::syntax::terms::Term<Cns>,
    state: &mut CompileState,
) -> core_lang::syntax::terms::Clause<Cns, Statement> {
    core_lang::syntax::terms::Clause {
        prdcns: Cns,
        xtor: clause.xtor,
        context: compile_context(clause.context),
        rhs: Rc::new(clause.rhs.compile_with_cont(cont, state)),
    }
}

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
        var: new_covar.clone(),
        chi: Chirality::Cns,
        ty: ty.clone(),
    });

    core_lang::syntax::terms::Clause {
        prdcns: Prd,
        xtor: clause.xtor,
        context: new_context,
        rhs: Rc::new(
            clause.rhs.compile_with_cont(
                core_lang::syntax::terms::XVar {
                    prdcns: Cns,
                    var: new_covar,
                    ty,
                }
                .into(),
                state,
            ),
        ),
    }
}
