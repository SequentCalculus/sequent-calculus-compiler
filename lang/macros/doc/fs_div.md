Create a [`core_lang::syntax::terms::op::FsOp`] with
[`core_lang::syntax::terms::op::BinOp::Div`], that is a focused divistion term.

```
use macros::fs_div;
use core_lang::syntax::terms::op::{BinOp,FsOp};
let div1 = fs_div!("x","y");
let div2 = FsOp{
    fst: "x".to_string(),
    op:BinOp::Div,
    snd: "y".to_string()
};
assert_eq!(div1,div2)
```
