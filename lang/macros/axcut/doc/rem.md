Create a [`axcut::syntax::statements::op::Op`] with
[`axcut::syntax::statements::op::BinOp::Rem`]. If `free_vars_next` is not
provided, it defaults to `None`

```
use axcut::syntax::statements::{
    exit::Exit,
    op::{BinOp, Op},
    Statement,
};
use axcut_macros::rem;
use std::{collections::HashSet, rc::Rc};

let rem1 = rem!(
    "x",
    "y",
    "z",
    Exit {
        var: "z".to_string()
    },
    ["z"]
);
let rem2 = Op {
    fst: "x".to_string(),
    op: BinOp::Rem,
    snd: "y".to_string(),
    var: "z".to_string(),
    next: Rc::new(Statement::from(Exit {
        var: "z".to_string(),
    })),
    free_vars_next: Some(HashSet::from(["z".to_string()])),
};
assert_eq!(rem1, rem2)
```
