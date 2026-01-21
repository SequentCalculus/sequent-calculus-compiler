Create a [`core_lang::syntax::terms::xvar::XVar`] with chrality
[`core_lang::syntax::terms::Prd`] If no type is provided, the variable will
default to [`core_lang::syntax::types::Ty::I64`]

```
use macros::{ty,var};
use core_lang::syntax::{types::Ty, terms::xvar::XVar};
let var1 = var!("x");
let var2 = var!("x",Ty::I64);
let var3 = XVar::var("x",Ty::I64);
assert_eq!(var1,var2);
assert_eq!(var2,var3);
```
