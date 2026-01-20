use core_lang::syntax::{
    Arguments,
    statements::{
        Call, Cut,
        ifc::{IfC, IfSort},
    },
    terms::{Term, xtor::Xtor, xvar::XVar},
    types::Ty,
};
use macros::{call, covar, ctor, cut, dtor, ifc, ty, var};
use std::rc::Rc;

#[test]
fn cut_macro() {
    let cut1 = cut!(var!("x"), covar!("a"));
    let cut2 = Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64);
    assert_eq!(cut1, cut2)
}

#[test]
fn list_int() {
    let list1 = ctor!(
        "Cons",
        [var!("x"), ctor!("Nil", [], ty!("ListInt"))],
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
    let fun1 = dtor!("apply", [var!("x")], ty!("FunI64I64"));
    let mut arguments = Arguments::default();
    arguments.add_prod(XVar::var("x", Ty::I64));
    let fun2 = Xtor::dtor("apply", arguments, Ty::Decl("FunI64I64".to_string()));
    assert_eq!(fun1, fun2)
}

#[test]
fn if_zero() {
    let if1 = ifc!(
        IfSort::Equal,
        var!("x"),
        var!("y"),
        cut!(var!("a"), covar!("b")),
        cut!(var!("c"), covar!("d"))
    );
    let if2 = IfC {
        sort: IfSort::Equal,
        fst: Rc::new(Term::from(XVar::var("x", ty!("int")))),
        snd: Some(Rc::new(Term::from(XVar::var("y", ty!("int"))))),
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

#[test]
fn call_print() {
    let call1 = call!("print", [var!("x")],);
    let call2 = Call {
        name: "print".to_string(),
        args: Arguments {
            entries: vec![Term::from(XVar::var("x", Ty::I64)).into()],
        },
        ty: Ty::I64,
    };
    assert_eq!(call1, call2);
}
