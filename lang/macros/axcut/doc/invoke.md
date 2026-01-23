Create a [`axcut::syntax::statements::invoke::Invoke`]. If no type is provided,
defaults to [`axcut::syntax::types::Ty::I64`]

```
use axcut_macros::invoke;
use axcut::syntax::{
    statements::invoke::Invoke,
    context::{ContextBinding, TypingContext, Chirality},
    types::Ty
};
let invoke1 = invoke!(
    "f",
    "apply",
    Ty::Decl("FunIntInt".to_string()),
    [ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64}],
);
let invoke2 = Invoke{
    var:"f".to_string(),
    tag:"apply".to_string(),
    ty:Ty::Decl("FunIntInt".to_string()),
    args:TypingContext{
        bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},
        ]
    }
};
assert_eq!(invoke1,invoke2);
```
