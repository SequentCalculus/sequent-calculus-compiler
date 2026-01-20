Create a [`core_lang::syntax::statements::call::FsCall`]

```
use macros::fs_call;
use core_lang::syntax::{
    context::{ContextBinding, TypingContext, Chirality},
    statements::call::FsCall,
    types::Ty
};

let call1 = fs_call!("exit",[ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64}]);
let call2 = FsCall{
    name:"exit".to_string(),
    args:TypingContext{ 
        bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64}
        ]
    }
};
assert_eq!(call1,call2)
```
