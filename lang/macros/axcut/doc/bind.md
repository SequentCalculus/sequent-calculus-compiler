Create a [`axcut::syntax::context::ContextBinding`]. If only a single argument
is provided, default to chirality [`axcut::syntax::context::Chirality::Ext`]
with type [`axcut::syntax::types::Ty::I64`]

```
use axcut::syntax::{
    context::{Chirality, ContextBinding},
    types::Ty,
};
use axcut_macros::bind;

let bind1 = bind!("x", Chirality::Ext, Ty::I64);
let bind2 = ContextBinding {
    var: "x".to_string(),
    chi: Chirality::Ext,
    ty: Ty::I64,
};
assert_eq!(bind1, bind2)
```
