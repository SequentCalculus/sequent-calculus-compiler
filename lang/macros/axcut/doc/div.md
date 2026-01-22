Create a [`axcut::syntax::statements::op::Op`] with
[`axcut::syntax::statements::op::BinOp::Div`]. If `free_vars_next` is not
provided, it defaults to `None`

```
use axcut_macros::div;
use axcut::syntax::{
    statements::{Statement, exit::Exit, op::{BinOp,Op}}
};
use std::{collections::HashSet, rc::Rc};

let div1 = div!("x","y","z",Exit{var:"z".to_string()},["z"]);
let div2 = Op{
    fst:"x".to_string(),
    op:BinOp::Div,
    snd:"y".to_string(),
    var:"z".to_string(),
    next:Rc::new(Statement::from(Exit{var:"z".to_string()})),
    free_vars_next:Some(HashSet::from(["z".to_string()])),
};
assert_eq!(div1,div2)
```
