Create a [`core_lang::syntax::terms::mu::Mu`] with chirality
[`core_lang::syntax::Cns`],\
that is, a mu-tilde-binding If no type is provided, defaults to
[`core_lang::syntax::types::Ty::I64`]

```
use macros::mutilde;
use core_lang::syntax::{
    Cns,
    terms::{xvar::XVar,mu::Mu},
    statements::{Exit,Statement},
    types::Ty,
};
use std::rc::Rc;

let mu1 = mutilde!("x",Exit::exit(XVar::var("x",Ty::I64),Ty::I64));
let mu2 = Mu{
    prdcns:Cns,
    variable:"x".to_string(),
    statement:Rc::new(Statement::from(Exit::exit(XVar::var("x",Ty::I64),Ty::I64))),
    ty:Ty::I64
};
assert_eq!(mu1,mu2)
```
