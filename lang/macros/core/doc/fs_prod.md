Create a [`core_lang::syntax::terms::op::FsOp`] with
[`core_lang::syntax::terms::op::BinOp::Prod`], that is a focused divistion term.

```
use core_lang::syntax::terms::op::{BinOp, FsOp};
use macros::fs_prod;
let prod1 = fs_prod!("x", "y");
let prod2 = FsOp {
    fst: "x".to_string(),
    op: BinOp::Prod,
    snd: "y".to_string(),
};
assert_eq!(prod1, prod2)
```
