Create a [`axcut::syntax::declaration::XtorSig`]

```
use axcut_macros::xtor_sig;
use axcut::syntax::{
    declaration::XtorSig,
    context::{ContextBinding,TypingContext,Chirality},
    types::Ty
};
let xtor1 = xtor_sig!("Cons", [
    ContextBinding {var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},
    ContextBinding {var:"xs".to_string(),chi:Chirality::Prd, ty:Ty::Decl("ListInt".to_string())}
]);
let xtor2 = XtorSig{
    name:"Cons".to_string(),
    args: TypingContext{
        bindings:vec![
            ContextBinding { var:"x".to_string(), chi:Chirality::Ext, ty:Ty::I64 },
            ContextBinding { var:"xs".to_string(), chi:Chirality::Prd, ty:Ty::Decl("ListInt".to_string()) },
        ]
    }
};
assert_eq!(xtor1,xtor2)
```
