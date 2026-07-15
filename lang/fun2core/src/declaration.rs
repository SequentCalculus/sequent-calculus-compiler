//! This module defines the translation of constructors of data and destructors of codata type
//! declarations.

use crate::context::compile_context;
use crate::program::build_type_param_subst;
use crate::types::{compile_ty, compile_type_params};
use core_lang::syntax::names::Identifier;
use fun::syntax::fresh_covar;
use std::collections::HashMap;
use std::rc::Rc;

/// This function converts [constructors in Fun](fun::syntax::declarations::CtorSig) to
/// [constructors in Core](core_lang::syntax::declaration::XtorSig), replacing type parameters with
/// the given core identifiers.
/// - `ctor` is the Fun constructor to translate.
/// - `type_params` maps Fun type parameter names to fresh Core identifiers.
pub fn compile_ctor(
    ctor: fun::syntax::declarations::CtorSig,
    type_params: Rc<HashMap<String, Identifier>>,
    max_id: &mut usize,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Data> {
    let ctor_type_params = compile_type_params(&ctor.type_params, max_id);
    let type_params_subst: Rc<HashMap<String, Identifier>> = Rc::new(
        (*type_params)
            .clone()
            .into_iter()
            .chain(build_type_param_subst(
                &ctor.type_params.bindings,
                &ctor_type_params,
            ))
            .collect(),
    );

    core_lang::syntax::declaration::XtorSig {
        xtor: core_lang::syntax::declaration::Data,
        name: Identifier::new(ctor.name),
        type_params: ctor_type_params,
        args: compile_context(ctor.args, type_params_subst),
    }
}

/// This function converts [destructors in Fun](fun::syntax::declarations::DtorSig) to
/// [destructors in Core](core_lang::syntax::declaration::XtorSig), replacing type parameters with
/// the given core identifiers.
/// - `dtor` is the Fun destructor to translate.
/// - `type_params` maps Fun type parameter names to fresh Core identifiers.
pub fn compile_dtor(
    dtor: fun::syntax::declarations::DtorSig,
    type_params: Rc<HashMap<String, Identifier>>,
    max_id: &mut usize,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Codata> {
    let new_covar = fresh_covar(&mut dtor.args.vars());
    let mut new_args = compile_context(dtor.args, type_params.clone());

    let dtor_type_params = compile_type_params(&dtor.type_params, max_id);
    let type_params_subst: Rc<HashMap<String, Identifier>> = Rc::new(
        (*type_params)
            .clone()
            .into_iter()
            .chain(build_type_param_subst(
                &dtor.type_params.bindings,
                &dtor_type_params,
            ))
            .collect(),
    );

    new_args
        .bindings
        .push(core_lang::syntax::context::ContextBinding {
            var: core_lang::syntax::names::Identifier::new(new_covar),
            chi: core_lang::syntax::context::Chirality::Cns,
            ty: compile_ty(&dtor.cont_ty, type_params_subst),
        });
    core_lang::syntax::declaration::XtorSig {
        xtor: core_lang::syntax::declaration::Codata,
        name: Identifier::new(dtor.name),
        type_params: dtor_type_params,
        args: new_args,
    }
}
