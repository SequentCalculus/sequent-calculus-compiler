Create a [`axcut::syntax::declaration::TypeDeclaration`]

```
use axcut_macros::ty_decl;
use axcut::syntax::{
    declaration::{XtorSig,TypeDeclaration},
    context::{TypingContext,ContextBinding,Chirality},
    types::Ty
};
let decl1 = ty_decl!("ListInt", [
    XtorSig{
        name:"Nil".to_string(),
        args:TypingContext{
            bindings:vec![]
        }
    },
    XtorSig{
        name:"Cons".to_string(),
        args:TypingContext{
            bindings:vec![
                ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},
                ContextBinding{var:"xs".to_string(),chi:Chirality::Prd,ty:Ty::Decl("ListInt".to_string())}
            ]
        }
    }
]);
let decl2 = TypeDeclaration{
    name:"ListInt".to_string(),
    xtors: vec![
        XtorSig{
            name:"Nil".to_string(),
            args:TypingContext{ bindings:vec![] } 
        },
        XtorSig{
            name:"Cons".to_string(),
            args:TypingContext{
                bindings:vec![
                    ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},
                    ContextBinding{var:"xs".to_string(),chi:Chirality::Prd,ty:Ty::Decl("ListInt".to_string())}
                ]
            }
        }
    ]
};
```
