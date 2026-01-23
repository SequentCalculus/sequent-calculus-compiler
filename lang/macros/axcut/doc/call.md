Create a [`axcut::syntax::statements::call::Call`]

```
use axcut_macros::call;
use axcut::syntax::{
    statements::call::Call,
    context::{ContextBinding,Chirality, TypingContext},
    types::Ty
};

let call1 = call!("exit",[ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64}]);
let call2 = Call{
    label:"exit".to_string(),
    args:TypingContext{
        bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64}
        ]
    }
    
};
assert_eq!(call1,call2)
```
