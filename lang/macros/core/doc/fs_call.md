Create a [`core_lang::syntax::statements::call::FsCall`]

```
use core_lang::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    statements::call::FsCall,
    types::Ty,
};
use core_macros::fs_call;

let call1 = fs_call!(
    "exit",
    [ContextBinding {
        var: "x".to_string(),
        chi: Chirality::Prd,
        ty: Ty::I64
    }]
);
let call2 = FsCall {
    name: "exit".to_string(),
    args: TypingContext {
        bindings: vec![ContextBinding {
            var: "x".to_string(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        }],
    },
};
assert_eq!(call1, call2)
```
