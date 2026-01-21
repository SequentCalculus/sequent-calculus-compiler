Create a [`core_lang::syntax::terms::op::FsOp`] with
[`core_lang::syntax::terms::op::BinOp::Rem`], that is a focused divistion term.

```
use core_lang::syntax::terms::op::{BinOp, FsOp};
use macros::fs_rem;
let rem1 = fs_rem!("x", "y");
let rem2 = FsOp {
    fst: "x".to_string(),
    op: BinOp::Rem,
    snd: "y".to_string(),
};
assert_eq!(rem1, rem2)
```
