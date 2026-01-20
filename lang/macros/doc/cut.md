Create a [`core_lang::syntax::statements::Cut`] with given arguments if no type
is provided, the cut type will default to [`core_lang::syntax::types::Ty::I64`]

```
use macros::cut;
use core_lang::syntax::{ statements::Cut, terms::xvar::XVar,types::Ty};
let cut1 = cut!(XVar::var("x",Ty::I64),XVar::covar("a",Ty::I64));
let cut2 = cut!(XVar::var("x",Ty::I64),XVar::covar("a",Ty::I64),Ty::I64);
let cut3 = Cut::new(XVar::var("x",Ty::I64),XVar::covar("a",Ty::I64),Ty::I64);
assert_eq!(cut1,cut2);
assert_eq!(cut2,cut3)
```
