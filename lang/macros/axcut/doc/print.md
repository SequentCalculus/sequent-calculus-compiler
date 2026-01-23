Create a [`axcut::syntax::statements::print::PrintI64`] without newline.
`free_vars_next` is optional and defaults to `None`

```
use axcut_macros::print_i64;
use axcut::syntax::{
    statements::{Statement, exit::Exit,print::PrintI64}
};
use std::{rc::Rc,collections::HashSet};

let print1 = print_i64!("x",Exit{var:"x".to_string()},["x"]);
let print2 = PrintI64{
    newline:false,
    var:"x".to_string(),
    next:Rc::new(Statement::from(Exit{var:"x".to_string()})),
    free_vars_next:Some(HashSet::from(["x".to_string()]))
};
assert_eq!(print1,print2)
```
