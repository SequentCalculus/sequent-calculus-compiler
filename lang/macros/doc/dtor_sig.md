Create a [`core_lang::syntax::declaration::DtorSig`]

```
use core_lang::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    declaration::{Codata, DtorSig},
    types::Ty,
};
use macros::dtor_sig;

let dtor1 = dtor_sig!(
    "apply",
    [
        ContextBinding {
            var: "x".to_string(),
            chi: Chirality::Prd,
            ty: Ty::I64
        },
        ContextBinding {
            var: "a".to_string(),
            chi: Chirality::Cns,
            ty: Ty::I64
        }
    ]
);
let dtor2 = DtorSig {
    xtor: Codata,
    name: "apply".to_string(),
    args: TypingContext {
        bindings: vec![
            ContextBinding {
                var: "x".to_string(),
                chi: Chirality::Prd,
                ty: Ty::I64,
            },
            ContextBinding {
                var: "a".to_string(),
                chi: Chirality::Cns,
                ty: Ty::I64,
            },
        ],
    },
};
assert_eq!(dtor1, dtor2)
```
