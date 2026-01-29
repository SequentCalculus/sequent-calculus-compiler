Create a [`core_lang::syntax::terms::FsMu`] with chirality
[`core_lang::syntax::Cns`] that is, a focused mu-tilde binding. If no type is
provided, [`core_lang::syntax::types::Ty::I64`] is used

```
use core_lang::syntax::{
    statements::{exit::FsExit, FsStatement},
    terms::{mu::FsMu, XVar},
    types::Ty,
    Cns,
};
use macros::fs_mutilde;
use std::rc::Rc;

let mu1 = fs_mutilde!("x", FsExit::exit("x"));
let mu2 = FsMu {
    prdcns: Cns,
    variable: "x".to_string(),
    statement: Rc::new(FsStatement::from(FsExit::exit("x"))),
    ty: Ty::I64,
};
assert_eq!(mu1, mu2)
```
