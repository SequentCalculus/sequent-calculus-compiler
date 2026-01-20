Create a [`core_lang::syntax::statements::op::FsOp`]

```
use macros::fs_op;
use core_lang::syntax::terms::op::{BinOp,FsOp};

let op1=fs_op!("x",BinOp::Prod,"y");
let op2= FsOp{
    fst:"x".to_string(),
    op:BinOp::Prod,
    snd:"y".to_string(),
};
assert_eq!(op1,op2)
```
