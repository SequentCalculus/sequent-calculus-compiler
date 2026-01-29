Create a [`core_lang::syntax::def::FsDef`]

```
use core_lang::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    def::FsDef,
    statements::{exit::FsExit, FsStatement},
    types::Ty,
};
use core_macros::fs_def;
use std::{collections::HashSet, rc::Rc};

let def1 = fs_def!(
    "exit",
    [ContextBinding {
        var: "x".to_string(),
        chi: Chirality::Prd,
        ty: Ty::I64
    }],
    FsExit::exit("x"),
    ["x"]
);
let def2 = FsDef {
    name: "exit".to_string(),
    context: TypingContext {
        bindings: vec![ContextBinding {
            var: "x".to_string(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        }],
    },
    body: FsStatement::from(FsExit::exit("x")),
    used_vars: HashSet::from(["x".to_string()]),
};
```
