#[must_use]
pub fn translate_ty(ty: core::syntax::Ty) -> axcut::syntax::Ty {
    match ty {
        core::syntax::Ty::Int() => axcut::syntax::Ty::Int,
        core::syntax::Ty::Decl(name) => axcut::syntax::Ty::Decl(name),
    }
}
