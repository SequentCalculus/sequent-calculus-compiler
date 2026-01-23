Create a [`axcut::syntax::statements::Clause`]

```
use axcut_macros::clause;
use axcut::syntax::{
    statements::{Statement, Clause, Exit},
    context::{TypingContext,ContextBinding,Chirality},
    types::Ty
};
use std::rc::Rc;

let clause1 = clause!("Cons",[
    ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},
    ContextBinding{var:"xs".to_string(), chi:Chirality::Prd,ty:Ty::Decl("ListInt".to_string())}
], Exit{ var:"x".to_string() } );
let clause2 = Clause{
    xtor: "Cons".to_string(),
    context: TypingContext{
        bindings: vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},
            ContextBinding{var:"xs".to_string(), chi:Chirality::Prd,ty:Ty::Decl("ListInt".to_string())}
        ]
    },
    body: Rc::new(Statement::from(Exit{ var:"x".to_string() }))
};
```
