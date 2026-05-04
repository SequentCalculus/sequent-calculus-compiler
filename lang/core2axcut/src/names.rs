//! This module defines the translation of names.

/// This function translates a name in focused [Core](core_lang) to a name in non-linearized
/// [AxCut](axcut).
/// - `identifier` is the name to translate.
pub fn shrink_identifier(
    identifier: core_lang::syntax::names::Identifier,
) -> axcut::syntax::names::Identifier {
    axcut::syntax::names::Identifier {
        name: identifier.name,
        id: identifier.id,
    }
}
