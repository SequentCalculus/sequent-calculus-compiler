Create a [`core_lang::syntax::terms::clause::FsClause`]

```
use macros::fs_clause;
use core_lang::syntax::{
    Prd,
    terms::clause::FsClause,
    context::{Chirality,ContextBinding,TypingContext},
    statements::{FsStatement,exit::FsExit},
    types::Ty
};
use std::rc::Rc;

let clause1 = fs_clause!(Prd,"apply",[
    ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
    ContextBinding{var:"a".to_string(),chi:Chirality::Cns, ty:Ty::I64}
],FsExit::exit("x"));
let clause2 = FsClause{
    prdcns:Prd,
    xtor:"apply".to_string(),
    context:TypingContext{
        bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
            ContextBinding{var:"a".to_string(),chi:Chirality::Cns, ty:Ty::I64}
        ]
    },
    body:Rc::new(FsStatement::from(FsExit::exit("x")))
};
assert_eq!(clause1,clause2);
```
