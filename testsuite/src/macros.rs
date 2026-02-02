use core_lang::syntax::{statements::Cut, terms::xvar::XVar, types::Ty};
use macros::cut;

#[test]
fn cut_macro() {
    let cut1 = cut!(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64);
    let cut2 = Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64);
    assert_eq!(cut1, cut2)
}
