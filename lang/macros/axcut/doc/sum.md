Create a [`axcut::syntax::statements::op::Op`] with
[`axcut::syntax::statements::op::BinOp::Sum`]. If `free_vars_next` is not
provided, it defaults to `None`

```
use axcut::syntax::statements::{
    exit::Exit,
    op::{BinOp, Op},
    Statement,
};
use axcut_macros::sum;
use std::{collections::HashSet, rc::Rc};

let sum1 = sum!(
    "x",
    "y",
    "z",
    Exit {
        var: "z".to_string()
    },
    ["z"]
);
let sum2 = Op {
    fst: "x".to_string(),
    op: BinOp::Sum,
    snd: "y".to_string(),
    var: "z".to_string(),
    next: Rc::new(Statement::from(Exit {
        var: "z".to_string(),
    })),
    free_vars_next: Some(HashSet::from(["z".to_string()])),
};
assert_eq!(sum1, sum2)
```
