Create a [`core_lang::syntax::declaration::CtorSig`]

```
use macros::ctor_sig;
use core_lang::syntax::{
    declaration::{Data,CtorSig},
    context::{TypingContext,ContextBinding,Chirality},
    types::Ty
};
let ctor1 = ctor_sig!("Cons",[
    ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
    ContextBinding{var:"xs".to_string(),chi:Chirality::Prd,ty:Ty::Decl("ListInt".to_string())}
]);
let ctor2 = CtorSig{
    xtor:Data,
    name:"Cons".to_string(),
    args:TypingContext{
        bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
            ContextBinding{var:"xs".to_string(),chi:Chirality::Prd,ty:Ty::Decl("ListInt".to_string())}
        ]
    }
};
assert_eq!(ctor1,ctor2)
```
