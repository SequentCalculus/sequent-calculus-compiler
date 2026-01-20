Create a [`core_lang::syntax::terms::xvar::XVar`] with chirality
[`core_lang::syntax::terms::Cns`] If no type is provided the covariable will
default to [`core_lang::syntax::types::ty::I64`]

```
use macros::covar;
use core_lang::syntax::{terms::xvar::XVar,types::Ty};
let covar1 = covar!("a");
let covar2 = covar!("a",Ty::I64);
let covar3 = XVar::covar("a",Ty::I64);
assert_eq!(covar1,covar2);
assert_eq!(covar2,covar3);
```
