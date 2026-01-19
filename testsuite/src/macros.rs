use core_lang::syntax::{
    Arguments,
    statements::{
        Cut,
        ifc::{IfC, IfSort},
    },
    terms::{xtor::Xtor, xvar::XVar},
    types::Ty,
};
use macros::{ctor, cut, dtor, ifc, ty};
use std::rc::Rc;

#[test]
fn cut_macro() {
    let cut1 = cut!(
        XVar::var("x", Ty::I64),
        XVar::covar("a", Ty::I64),
        ty!("int")
    );
    let cut2 = Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64);
    assert_eq!(cut1, cut2)
}

#[test]
fn list_int() {
    let list1 = ctor!(
        "Cons",
        [XVar::var("x", ty!("int")), ctor!("Nil", [], ty!("ListInt")),],
        ty!("ListInt")
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
    let fun1 = dtor!("apply", [XVar::var("x", ty!("int"))], ty!("FunI64I64"));
    let mut arguments = Arguments::default();
    arguments.add_prod(XVar::var("x", Ty::I64));
    let fun2 = Xtor::dtor("apply", arguments, Ty::Decl("FunI64I64".to_string()));
    assert_eq!(fun1, fun2)
}

#[test]
fn if_zero() {
    let if1 = ifc!(
        IfSort::Equal,
        XVar::var("x", ty!("int")),
        XVar::var("y", ty!("int")),
        cut!(
            XVar::var("a", ty!("int")),
            XVar::covar("b", ty!("int")),
            ty!("int")
        ),
        cut!(
            XVar::var("c", ty!("int")),
            XVar::covar("d", ty!("int")),
            ty!("int")
        )
    );
    let if2 = IfC {
        sort: IfSort::Equal,
        fst: XVar::var("x", ty!("int")),
        snd: Some(XVar::var("y", ty!("int"))),
        thenc: Rc::new(
            Cut::new(
                XVar::var("a", ty!("int")),
                XVar::covar("b", ty!("int")),
                ty!("int"),
            )
            .into(),
        ),
        elsec: Rc::new(
            Cut::new(
                XVar::var("c", ty!("int")),
                XVar::covar("d", ty!("int")),
                ty!("int"),
            )
            .into(),
        ),
    };
    assert_eq!(if1, if2)
}
