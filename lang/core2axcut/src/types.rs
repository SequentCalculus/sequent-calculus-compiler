//! This module defines the trivial translation on types.

/// This function translates a [Core](core_lang) type to an [AxCut](axcut) type.
/// - `ty` is the [Core](core_lang) type.
pub fn shrink_ty(ty: core_lang::syntax::Ty) -> axcut::syntax::Ty {
    match ty {
        core_lang::syntax::Ty::I64 => axcut::syntax::Ty::I64,
        core_lang::syntax::Ty::Decl(name) => axcut::syntax::Ty::Decl(name),
    }
}
