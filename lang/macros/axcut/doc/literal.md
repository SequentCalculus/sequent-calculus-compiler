Create a [`axcut::syntax::statements::Literal`]. If `free_vars_next` is not
provided, default to `None`

```
use axcut_macros::lit;
use axcut::syntax::statements::{Statement, literal::Literal,Exit};
use std::{collections::HashSet, rc::Rc};

let lit1 = lit!(1,"x",Exit{var:"x".to_string()},["x"]);
let lit2 = Literal{
    lit:1,
    var:"x".to_string(),
    next:Rc::new(Statement::from(Exit{var:"x".to_string()})),
    free_vars_next:Some(HashSet::from(["x".to_string()]))
};
assert_eq!(lit1,lit2);
```
