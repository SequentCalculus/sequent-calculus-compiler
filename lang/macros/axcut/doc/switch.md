Create a [`axcut::syntax::statements::Switch`]. `free_vars_clauses` and `ty` are
optional. `ty` defaults to [`axcut::syntax::types::Ty::I64`] and
`free_vars_clauses` defaults to `None`. It is not possible to skip `ty` and
provide `free_vars_clauses`. Instead use `axcut_macros::ty`] with argument
`"int"`.

```
use axcut_macros::switch;
use axcut::syntax::{
    statements::{Clause, Switch, Exit, Statement},
    context::{TypingContext,ContextBinding,Chirality},
    types::Ty
};
use std::{rc::Rc,collections::HashSet};

let switch1 = switch!("x",Ty::Decl("ListInt".to_string()),[
    Clause{
        xtor:"Nil".to_string(),
        context:TypingContext{bindings:vec![]},
        body:Rc::new(Statement::from(Exit{var:"x".to_string()}))
    },
    Clause{
        xtor:"Cons".to_string(),
        context:TypingContext{
            bindings:vec![
                ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},
                ContextBinding{var:"xs".to_string(),chi:Chirality::Prd,ty:Ty::Decl("ListInt".to_string())}
            ]
        },
            body:Rc::new(Statement::from(Exit { var:"x".to_string() }))
    }
],["x"]);
let switch2 = Switch{
    var:"x".to_string(),
    ty:Ty::Decl("ListInt".to_string()),
    clauses:vec![
        Clause{
            xtor:"Nil".to_string(),
            context:TypingContext{bindings:vec![]},
            body:Rc::new(Statement::from(Exit{var:"x".to_string()}))
        },
        Clause{
            xtor:"Cons".to_string(),
            context:TypingContext{
                bindings:vec![
                    ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},
                    ContextBinding{var:"xs".to_string(),chi:Chirality::Prd,ty:Ty::Decl("ListInt".to_string())}
                ]
            },
            body:Rc::new(Statement::from(Exit { var:"x".to_string() }))
        }
    ],
    free_vars_clauses:Some(HashSet::from(["x".to_string()]))
};
assert_eq!(switch1,switch2);
```
