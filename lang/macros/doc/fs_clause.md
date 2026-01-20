Create a [`core_lang::syntax::terms::clause::FsClause`]

```
use macros::fs_clause;
use core_lang::syntax::{
    Prd,
    terms::clause::FsClause,
    context::{Chirality,ContextBinding,TypingContext},
    types::Ty
};
use std::rc::Rc;

let clause1 = fs_clause!(Prd,"apply",[
    ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
    ContextBinding{var:"a".to_string(),chi:Chiralirty::Cns, ty:Ty::I64}
],FsExit::exit("x"));
let clause2 = FsClause{
    prdcns:Prd,
    xtor:"apply".to_string(),
    context:TypingContext{
        bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
            ContextBinding{var:"a".to_string(),chi:Chiralirty::Cns, ty:Ty::I64}
        ]
    }
    body:Rc::new(FsExit::exit("x"))
};
assert_eq!(clause1,clause2);
```
