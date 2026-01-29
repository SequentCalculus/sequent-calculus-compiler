Create a [`core_lang::syntax::statements::Exit`] if no return type is provided,
the type will default to `[core_lang::syntax::types::Ty::I64`]

```
use core_lang::syntax::{statements::Exit, terms::xvar::XVar, types::Ty};
use macros::exit;
let exit1 = exit!(XVar::var("x", Ty::I64), Ty::I64);
let exit2 = exit!(XVar::var("x", Ty::I64));
let exit3 = Exit::exit(XVar::var("x", Ty::I64), Ty::I64);
assert_eq!(exit1, exit2);
assert_eq!(exit2, exit3);
```
