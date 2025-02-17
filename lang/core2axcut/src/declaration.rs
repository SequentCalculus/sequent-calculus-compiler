use core_lang::syntax::declaration::{CodataDeclaration, DataCodata};

use crate::context::shrink_context;

pub fn shrink_sig<T: DataCodata>(
    sig: core_lang::syntax::declaration::XtorSig<T>,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::XtorSig {
    axcut::syntax::XtorSig {
        name: sig.name,
        args: shrink_context(sig.args, codata_types),
    }
}

pub fn shrink_declaration<T: DataCodata>(
    declaration: core_lang::syntax::declaration::TypeDeclaration<T>,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::TypeDeclaration {
    axcut::syntax::TypeDeclaration {
        name: declaration.name,
        xtors: declaration
            .xtors
            .into_iter()
            .map(|xtor| shrink_sig(xtor, codata_types))
            .collect(),
    }
}
