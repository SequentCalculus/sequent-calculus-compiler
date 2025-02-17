pub fn shrink_ty(ty: core_lang::syntax::Ty) -> axcut::syntax::Ty {
    match ty {
        core_lang::syntax::Ty::I64 => axcut::syntax::Ty::I64,
        core_lang::syntax::Ty::Decl(name) => axcut::syntax::Ty::Decl(name),
    }
}
