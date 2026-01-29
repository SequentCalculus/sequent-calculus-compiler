Create a [`core_lang::syntax::terms::op::FsOp`] with
[`core_lang::syntax::terms::op::BinOp::Sum`], that is a focused divistion term.

```
use core_lang::syntax::terms::op::{BinOp, FsOp};
use core_macros::fs_sum;
let sum1 = fs_sum!("x", "y");
let sum2 = FsOp {
    fst: "x".to_string(),
    op: BinOp::Sum,
    snd: "y".to_string(),
};
assert_eq!(sum1, sum2)
```
