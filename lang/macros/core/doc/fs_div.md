Create a [`core_lang::syntax::terms::op::FsOp`] with
[`core_lang::syntax::terms::op::BinOp::Div`], that is a focused divistion term.

```
use core_lang::syntax::terms::op::{BinOp, FsOp};
use core_macros::fs_div;
let div1 = fs_div!("x", "y");
let div2 = FsOp {
    fst: "x".to_string(),
    op: BinOp::Div,
    snd: "y".to_string(),
};
assert_eq!(div1, div2)
```
