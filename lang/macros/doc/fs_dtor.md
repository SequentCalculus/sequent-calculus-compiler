Create a [`core_lang::syntax::terms::xtor::FsXtor`] with chirality
[`core_lang::syntax::Cns`], that is, a focussed destructor

```
use macros::fs_dtor;
use core_lang::syntax::{
    Cns,
    context::{ContextBinding, Chirality, TypingContext},
    terms::xtor::FsXtor,
    types::Ty
};
let xtor1 = fs_dtor!("apply",[
    ContextBinding{var:"x".to_string(), chi:Chirality::Prd, ty:Ty::I64},
], Ty::Decl("FunIntInt".to_string()));
let xtor2 = FsXtor{
    prdcns:Cns,
    id:"apply".to_string(),
    args:TypingContext{
        bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
        ]
    },
    ty:Ty::Decl("FunIntInt".to_string())
};
assert_eq!(xtor1,xtor2);
```
