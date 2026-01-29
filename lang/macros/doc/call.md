Create a [`core_lang::syntax::statements::Call`] if no return type is provided
it will default to [`core_lang::syntax::types::Ty::I64`]

```
use core_lang::syntax::{
    arguments::{Argument, Arguments},
    statements::Call,
    terms::{xvar::XVar, Term},
    types::Ty,
};
use macros::call;
let call1 = call!("print", [XVar::var("x", Ty::I64)], Ty::I64);
let call2 = call!("print", [XVar::var("x", Ty::I64)]);
let call3 = Call {
    name: "print".to_string(),
    args: Arguments {
        entries: Vec::from([Argument::from(Term::from(XVar::var("x", Ty::I64)))]),
    },
    ty: Ty::I64,
};
assert_eq!(call1, call2);
assert_eq!(call2, call3)
```
