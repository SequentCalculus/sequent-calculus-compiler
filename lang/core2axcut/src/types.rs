#[must_use]
pub fn translate_ty(ty: core::syntax_var::Ty) -> axcut::syntax::Ty {
    match ty {
        core::syntax_var::Ty::Int => axcut::syntax::Ty::Int,
        core::syntax_var::Ty::Decl(name) => axcut::syntax::Ty::Decl(name),
    }
}
