Create a [`axcut::syntax::declaration::XtorSig`]

```
use axcut::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    declaration::XtorSig,
    types::Ty,
};
use axcut_macros::xtor_sig;
let xtor1 = xtor_sig!(
    "Cons",
    [
        ContextBinding {
            var: "x".to_string(),
            chi: Chirality::Ext,
            ty: Ty::I64
        },
        ContextBinding {
            var: "xs".to_string(),
            chi: Chirality::Prd,
            ty: Ty::Decl("ListInt".to_string())
        }
    ]
);
let xtor2 = XtorSig {
    name: "Cons".to_string(),
    args: TypingContext {
        bindings: vec![
            ContextBinding {
                var: "x".to_string(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            },
            ContextBinding {
                var: "xs".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("ListInt".to_string()),
            },
        ],
    },
};
assert_eq!(xtor1, xtor2)
```
