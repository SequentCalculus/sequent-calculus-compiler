Create a [`core_lang::syntax::terms::FsMu`] with chirality
[`core_lang::syntax::Prd`] that is, a focused mu binding. If no type is
provided, [`core_lang::syntax::types::Ty::I64`] is used

```
use macros::fs_mu;
use core_lang::syntax::{
    Prd, 
    statements::{FsExit,FsStatement}, 
    terms::{XVar,mu::FsMu},
    types::Ty
};
use std::rc::Rc;

let mu1 = fs_mu!("a",FsExit::exit("a"));
let mu2 = FsMu{
    prdcns:Prd, 
    variable:"a".to_string(),
    statement:Rc::new(FsStatement::from(FsExit::exit("a"))),
    ty:Ty::I64
};
assert_eq!(mu1,mu2)
```
