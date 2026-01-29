Create a [`axcut::syntax::statements::let::Let`]. `free_vars_next` and `ty` are
optional, `free_vars_next` defaults to `None` and `ty` defaults to
[`axcut::syntax::types::Ty::I64`]

```
use axcut::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    statements::{Exit, Let, Statement},
    types::Ty,
};
use axcut_macros::letin;
use std::{collections::HashSet, rc::Rc};

let let1 = letin!(
    "x",
    Ty::I64,
    "Ret",
    [ContextBinding {
        var: "y".to_string(),
        chi: Chirality::Ext,
        ty: Ty::I64
    }],
    Exit {
        var: "x".to_string()
    },
    ["x"]
);
let let2 = Let {
    var: "x".to_string(),
    ty: Ty::I64,
    tag: "Ret".to_string(),
    args: TypingContext {
        bindings: vec![ContextBinding {
            var: "y".to_string(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        }],
    },
    next: Rc::new(Statement::from(Exit {
        var: "x".to_string(),
    })),
    free_vars_next: Some(HashSet::from(["x".to_string()])),
};
assert_eq!(let1, let2);
```
