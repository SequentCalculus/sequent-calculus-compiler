//! This module defines the translation of constructors of data and destructors of codata type
//! declarations.

use crate::context::compile_context_with_subst;
use crate::types::compile_ty_with_subst;
use core_lang::syntax::names::Identifier;
use fun::syntax::fresh_covar;
use std::collections::HashMap;

/// This function converts [constructors in Fun](fun::syntax::declarations::CtorSig) to
/// [constructors in Core](core_lang::syntax::declaration::XtorSig).
pub fn compile_ctor(
    ctor: fun::syntax::declarations::CtorSig,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Data> {
    compile_ctor_with_subst(ctor, &HashMap::new())
}

/// This function converts [constructors in Fun](fun::syntax::declarations::CtorSig) to
/// [constructors in Core](core_lang::syntax::declaration::XtorSig), replacing type parameters with
/// the given core identifiers.
/// - `ctor` is the Fun constructor to translate.
/// - `type_params` maps Fun type parameter names to fresh Core identifiers.
pub fn compile_ctor_with_subst(
    ctor: fun::syntax::declarations::CtorSig,
    type_params: &HashMap<String, Identifier>,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Data> {
    core_lang::syntax::declaration::XtorSig {
        xtor: core_lang::syntax::declaration::Data,
        name: Identifier::new(ctor.name),
        args: compile_context_with_subst(ctor.args, type_params),
    }
}

/// This function converts [constructors in Fun](fun::syntax::declarations::DtorSig) to
/// [constructors in Core](core_lang::syntax::declaration::XtorSig).
pub fn compile_dtor(
    dtor: fun::syntax::declarations::DtorSig,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Codata> {
    compile_dtor_with_subst(dtor, &HashMap::new())
}

/// This function converts [destructors in Fun](fun::syntax::declarations::DtorSig) to
/// [destructors in Core](core_lang::syntax::declaration::XtorSig), replacing type parameters with
/// the given core identifiers.
/// - `dtor` is the Fun destructor to translate.
/// - `type_params` maps Fun type parameter names to fresh Core identifiers.
pub fn compile_dtor_with_subst(
    dtor: fun::syntax::declarations::DtorSig,
    type_params: &HashMap<String, Identifier>,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Codata> {
    let new_covar = fresh_covar(&mut dtor.args.vars());
    let mut new_args = compile_context_with_subst(dtor.args, type_params);

    new_args
        .bindings
        .push(core_lang::syntax::context::ContextBinding {
            var: core_lang::syntax::names::Identifier::new(new_covar),
            chi: core_lang::syntax::context::Chirality::Cns,
            ty: compile_ty_with_subst(&dtor.cont_ty, type_params),
        });
    core_lang::syntax::declaration::XtorSig {
        xtor: core_lang::syntax::declaration::Codata,
        name: Identifier::new(dtor.name),
        args: new_args,
    }
}
