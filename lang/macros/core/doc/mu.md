Create a [`core_lang::syntax::terms::mu::Mu`] with chirality
[`core_lang::syntax::Prd`],\
that is, a mu-binding If no type is provided, defaults to
[`core_lang::syntax::types::Ty::I64`]

```
use core_lang::syntax::{
    statements::{Exit, Statement},
    terms::{mu::Mu, xvar::XVar},
    types::Ty,
    Prd,
};
use core_macros::mu;
use std::rc::Rc;

let mu1 = mu!("a", Exit::exit(XVar::var("x", Ty::I64), Ty::I64));
let mu2 = Mu {
    prdcns: Prd,
    variable: "a".to_string(),
    statement: Rc::new(Statement::from(Exit::exit(
        XVar::var("x", Ty::I64),
        Ty::I64,
    ))),
    ty: Ty::I64,
};
assert_eq!(mu1, mu2)
```
