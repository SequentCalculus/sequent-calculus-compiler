//! This module defines the translation of data and codata type declarations.

use core_lang::syntax::declaration::{CodataDeclaration, Polarity};

use crate::context::shrink_context;

/// This function translates an xtor, i.e., a constructor or destructor, in [Core](core_lang) to
/// one in [AxCut](axcut). It essentially consists of translating the parameters.
/// - `xtor` is the xtor to translate.
/// - `codata_types` is the list of codata types in the corresponding [Core](core_lang) program.
pub fn shrink_xtor<P: Polarity>(
    xtor: core_lang::syntax::declaration::XtorSig<P>,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::XtorSig {
    axcut::syntax::XtorSig {
        name: xtor.name.name,
        args: shrink_context(xtor.args, codata_types),
    }
}

/// This function translates a type declaration in [Core](core_lang) to one in [AxCut](axcut). It
/// essentially consists of translating the xtors.
/// - `declaration` is the type declaration to translate.
/// - `codata_types` is the list of codata types in the corresponding [Core](core_lang) program.
pub fn shrink_declaration<P: Polarity>(
    declaration: core_lang::syntax::declaration::TypeDeclaration<P>,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::TypeDeclaration {
    axcut::syntax::TypeDeclaration {
        name: declaration.name.name,
        xtors: declaration
            .xtors
            .into_iter()
            .map(|xtor| shrink_xtor(xtor, codata_types))
            .collect(),
    }
}
