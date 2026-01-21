Create a [`core_lang::syntax::declaration::DtorSig`]

```
use macros::dtor_sig;
use core_lang::syntax::{
    declaration::{Codata,DtorSig},
    context::{TypingContext,ContextBinding,Chirality},
    types::Ty,
};

let dtor1 = dtor_sig!("apply",[
    ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
    ContextBinding{var:"a".to_string(),chi:Chirality::Cns,ty:Ty::I64}
]);
let dtor2 = DtorSig{
    xtor:Codata,
    name:"apply".to_string(),
    args:TypingContext{
        bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
            ContextBinding{var:"a".to_string(),chi:Chirality::Cns,ty:Ty::I64}
        ]
    }
};
assert_eq!(dtor1,dtor2)
```
