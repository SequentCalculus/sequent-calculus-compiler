Create a [`axcut::syntax::statements::print::PrintI64`] with newline
`free_vars_next` is optional and defaults to `None`

```
use axcut_macros::println_i64;
use axcut::syntax::{
    statements::{Statement, exit::Exit,print::PrintI64}
};
use std::{rc::Rc,collections::HashSet};

let print1 = println_i64!("x",Exit{var:"x".to_string()},["x".to_string()]);
let print2 = PrintI64{
    newline:true,
    var:"x".to_string(),
    next:Rc::new(Statement::from(Exit{var:"x".to_string()})),
    free_vars_next:Some(HashSet::from(["x".to_string()]))
};
assert_eq!(print1,print2)
```
