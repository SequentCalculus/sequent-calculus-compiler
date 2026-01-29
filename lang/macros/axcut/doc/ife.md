Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
[`axcut::syntax::statements::ifc::IfSort::Equal`]. `snd` is optional and
defaults to `None`

```
use axcut::syntax::statements::{
    ifc::{IfC, IfSort},
    Exit, Statement,
};
use axcut_macros::ife;
use std::rc::Rc;

let if1 = ife!(
    "x",
    "y",
    Exit {
        var: "x".to_string()
    },
    Exit {
        var: "y".to_string()
    }
);
let if2 = IfC {
    sort: IfSort::Equal,
    fst: "x".to_string(),
    snd: Some("y".to_string()),
    thenc: Rc::new(Statement::from(Exit {
        var: "x".to_string(),
    })),
    elsec: Rc::new(Statement::from(Exit {
        var: "y".to_string(),
    })),
};
assert_eq!(if1, if2);
```
