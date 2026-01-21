Create a [`core_lang::syntax::terms::xtor::FsXtor`] with chirality
[`core_lang::syntax::Prd`], that is, a focussed constructor

```
use core_lang::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    terms::xtor::FsXtor,
    types::Ty,
    Prd,
};
use macros::fs_ctor;
let xtor1 = fs_ctor!(
    "Cons",
    [
        ContextBinding {
            var: "x".to_string(),
            chi: Chirality::Prd,
            ty: Ty::I64
        },
        ContextBinding {
            var: "xs".to_string(),
            chi: Chirality::Prd,
            ty: Ty::Decl("ListInt".to_string())
        }
    ],
    Ty::Decl("ListInt".to_string())
);
let xtor2 = FsXtor {
    prdcns: Prd,
    id: "Cons".to_string(),
    args: TypingContext {
        bindings: vec![
            ContextBinding {
                var: "x".to_string(),
                chi: Chirality::Prd,
                ty: Ty::I64,
            },
            ContextBinding {
                var: "xs".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("ListInt".to_string()),
            },
        ],
    },
    ty: Ty::Decl("ListInt".to_string()),
};
assert_eq!(xtor1, xtor2);
```
