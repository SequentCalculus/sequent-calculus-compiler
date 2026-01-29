Create a [`core_lang::syntax::statements::cut::FsCut`] if no type is provided,
default to [`core_lang::syntax::types::Ty::I64`]

```
use core_lang::syntax::{
    statements::FsCut,
    terms::{FsTerm, XVar},
    types::Ty,
};
use core_macros::fs_cut;
use std::rc::Rc;

let cut1 = fs_cut!(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64));
let cut2 = FsCut {
    producer: Rc::new(FsTerm::from(XVar::var("x", Ty::I64))),
    consumer: Rc::new(FsTerm::from(XVar::covar("a", Ty::I64))),
    ty: Ty::I64,
};
assert_eq!(cut1, cut2)
```
