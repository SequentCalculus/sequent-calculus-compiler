Create a [`axcut::syntax::statements::op::Op`] with
[`axcut::syntax::statements::op::BinOp::Sub`]. If `free_vars_next` is not
provided, it defaults to `None`

```
use axcut_macros::sub;
use axcut::syntax::{
    statements::{Statement, exit::Exit, op::{BinOp,Op}}
};
use std::{collections::HashSet, rc::Rc};

let sub1 = sub!("x","y","z",Exit{var:"z".to_string()},["z"]);
let sub2 = Op{
    fst:"x".to_string(),
    op:BinOp::Sub,
    snd:"y".to_string(),
    var:"z".to_string(),
    next:Rc::new(Statement::from(Exit{var:"z".to_string()})),
    free_vars_next:Some(HashSet::from(["z".to_string()])),
};
assert_eq!(sub1,sub2)
```
