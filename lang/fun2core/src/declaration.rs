use crate::{context::compile_context, types::compile_ty};
use core_lang::syntax::fresh_covar;

pub fn compile_ctor(
    ctor: fun::syntax::declarations::CtorSig,
) -> core_lang::syntax::declaration::XtorSig<core_lang::syntax::declaration::Data> {
    core_lang::syntax::declaration::XtorSig {
        xtor: core_lang::syntax::declaration::Data,
        name: ctor.name,
        args: compile_context(ctor.args),
    }
}

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
        name: dtor.name,
        args: new_args,
    }
}
