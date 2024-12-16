use crate::context::translate_context;
use core_lang::syntax::declaration::{CodataDeclaration, DataCodata};

#[must_use]
pub fn translate_sig<T: DataCodata>(
    sig: core_lang::syntax::declaration::XtorSig<T>,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::XtorSig {
    axcut::syntax::XtorSig {
        name: sig.name,
        args: translate_context(sig.args, codata_types),
    }
}

pub fn translate_declaration<T: DataCodata>(
    declaration: core_lang::syntax::declaration::TypeDeclaration<T>,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::TypeDeclaration {
    axcut::syntax::TypeDeclaration {
        name: declaration.name,
        xtors: declaration
            .xtors
            .into_iter()
            .map(|xtor| translate_sig(xtor, codata_types))
            .collect(),
    }
}
