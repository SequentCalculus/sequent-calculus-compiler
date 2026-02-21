//! This module defines the translation of constructors of data and destructors of codata type
//! declarations.

use crate::{context::compile_context, types::compile_ty};
use core_lang::syntax::{fresh_covar, names::Ident};

/// This function converts [constructors in Fun](fun::syntax::declarations::CtorSig) to
/// [constructors in Core](core_lang::syntax::declaration::XtorSig).
pub fn compile_ctor(
    ctor: fun::syntax::declarations::CtorSig,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Data> {
    core_lang::syntax::declaration::XtorSig {
        xtor: core_lang::syntax::declaration::Data,
        name: Ident::new_with_zero(&ctor.name),
        args: compile_context(ctor.args),
    }
}

/// This function converts [constructors in Fun](fun::syntax::declarations::DtorSig) to
/// [constructors in Core](core_lang::syntax::declaration::XtorSig).
pub fn compile_dtor(
    dtor: fun::syntax::declarations::DtorSig,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Codata> {
    let mut new_args = compile_context(dtor.args);

    let new_covar = fresh_covar(&mut new_args.vars().into_iter().collect());

    new_args
        .bindings
        .push(core_lang::syntax::context::ContextBinding {
            var: new_covar,
            chi: core_lang::syntax::context::Chirality::Cns,
            ty: compile_ty(&dtor.cont_ty),
        });
    core_lang::syntax::declaration::XtorSig {
        xtor: core_lang::syntax::declaration::Codata,
        name: Ident::new_with_zero(&dtor.name),
        args: new_args,
    }
}
