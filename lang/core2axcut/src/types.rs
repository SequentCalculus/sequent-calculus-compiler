#[must_use]
pub fn translate_ty(ty: core::syntax_var::Ty) -> axcut::syntax::Ty {
    match ty {
        core::syntax_var::Ty::Int => axcut::syntax::Ty::Int,
        core::syntax_var::Ty::Decl(name) => axcut::syntax::Ty::Decl(name),
    }
}

#[cfg(test)]
mod type_tests {

    use super::translate_ty;

    #[test]
    fn translate_int() {
        let result = translate_ty(core::syntax_var::Ty::Int);
        let expected = axcut::syntax::types::Ty::Int;
        assert_eq!(result, expected)
    }

    #[test]
    fn translate_list() {
        let result = translate_ty(core::syntax_var::Ty::Decl("ListInt".to_owned()));
        let expected = axcut::syntax::types::Ty::Decl("ListInt".to_owned());
        assert_eq!(result, expected)
    }
}
