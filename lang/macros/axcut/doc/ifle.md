Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
[`axcut::syntax::statements::ifc::IfSort::LessOrEqual`]. `snd` is optional and
defaults to `None`

```
use axcut_macros::ifle;
use axcut::syntax::{
    statements::{Statement, Exit, ifc::{IfSort, IfC}},
};
use std::rc::Rc;

let if1 = ifle!("x","y",Exit{var:"x".to_string()},Exit{var:"y".to_string()});
let if2 = IfC{
    sort:IfSort::LessOrEqual,
    fst:"x".to_string(),
    snd:Some("y".to_string()),
    thenc:Rc::new(Statement::from(Exit{var:"x".to_string()})),
    elsec:Rc::new(Statement::from(Exit{var:"y".to_string()}))
};
assert_eq!(if1,if2);
```
