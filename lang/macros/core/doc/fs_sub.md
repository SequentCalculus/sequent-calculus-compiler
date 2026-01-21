Create a [`core_lang::syntax::terms::op::FsOp`] with
[`core_lang::syntax::terms::op::BinOp::Sub`], that is a focused divistion term.

```
use core_lang::syntax::terms::op::{BinOp, FsOp};
use macros::fs_sub;
let sub1 = fs_sub!("x", "y");
let sub2 = FsOp {
    fst: "x".to_string(),
    op: BinOp::Sub,
    snd: "y".to_string(),
};
assert_eq!(sub1, sub2)
```
