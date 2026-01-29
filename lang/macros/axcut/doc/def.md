Create a [`axcut::syntax::def::Def`]. `used_vars` is optional and defaults to
`HashSet::new()`

```
use axcut::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    def::Def,
    statements::{Exit, Statement},
    types::Ty,
};
use axcut_macros::def;
use std::collections::HashSet;

let def1 = def!(
    "exit",
    [ContextBinding {
        var: "x".to_string(),
        chi: Chirality::Ext,
        ty: Ty::I64
    }],
    Exit {
        var: "x".to_string()
    },
    ["x"]
);
let def2 = Def {
    name: "exit".to_string(),
    context: TypingContext {
        bindings: vec![ContextBinding {
            var: "x".to_string(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        }],
    },
    body: Statement::from(Exit {
        var: "x".to_string(),
    }),
    used_vars: HashSet::from(["x".to_string()]),
};
assert_eq!(def1, def2);
```
