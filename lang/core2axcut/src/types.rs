#[must_use]
pub fn translate_ty(ty: core_lang::syntax::Ty) -> axcut::syntax::Ty {
    match ty {
        core_lang::syntax::Ty::Int => axcut::syntax::Ty::Int,
        core_lang::syntax::Ty::Decl(name) => axcut::syntax::Ty::Decl(name),
    }
}
