use core_lang::syntax::{
    Arguments,
    statements::Cut,
    terms::{xtor::Xtor, xvar::XVar},
    types::Ty,
};
use macros::{ctor, cut, dtor};

#[test]
fn cut_macro() {
    let cut1 = cut!(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64);
    let cut2 = Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64);
    assert_eq!(cut1, cut2)
}

#[test]
fn list_int() {
    let list1 = ctor!(
        "Cons",
        [
            XVar::var("x", Ty::I64),
            ctor!("Nil", [], Ty::Decl("ListInt".to_string())),
        ],
        Ty::Decl("ListInt".to_string())
    );
    let mut arguments = Arguments::default();
    arguments.add_prod(XVar::var("x", Ty::I64));
    arguments.add_prod(Xtor::ctor(
        "Nil",
        Arguments::default(),
        Ty::Decl("ListInt".to_string()),
    ));
    let list2 = Xtor::ctor("Cons", arguments, Ty::Decl("ListInt".to_string()));
    assert_eq!(list1, list2)
}

#[test]
fn fun_int() {
    let fun1 = dtor!(
        "apply",
        [XVar::var("x", Ty::I64)],
        Ty::Decl("FunI64I64".to_string())
    );
    let mut arguments = Arguments::default();
    arguments.add_prod(XVar::var("x", Ty::I64));
    let fun2 = Xtor::dtor("apply", arguments, Ty::Decl("FunI64I64".to_string()));
    assert_eq!(fun1, fun2)
}
