//! This module defines the translation of data and codata type declarations.

use core_lang::syntax::declaration::{CodataDeclaration, DataCodata};

use crate::context::shrink_context;

/// This function translates an xtor, i.e., a constructor or destructor, in [Core](core_lang) to
/// one in [AxCut](axcut). It essentially consists of translating the parameters.
/// - `sig` is the xtor to translate.
/// - `codata_types` is the list of codata types in the corresponding [Core](core_lang) program.
pub fn shrink_sig<T: DataCodata>(
    sig: core_lang::syntax::declaration::XtorSig<T>,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::XtorSig {
    axcut::syntax::XtorSig {
        name: sig.name,
        args: shrink_context(sig.args, codata_types),
    }
}

/// This function translates a type declaration in [Core](core_lang) to one in [AxCut](axcut). It
/// essentially consists of translating the xtors.
/// - `declaration` is the type declaration to translate.
/// - `codata_types` is the list of codata types in the corresponding [Core](core_lang) program.
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
