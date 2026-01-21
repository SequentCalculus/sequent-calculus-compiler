Create a [`core_lang::syntax::declaration::DataDeclaration`]

```
use macros::data;
use core_lang::syntax::{
    declaration::{Data,DataDeclaration,XtorSig},
    context::{TypingContext,ContextBinding,Chirality},
    types::Ty
};
let decl1 = data!("List",[
        XtorSig{
            xtor:Data,
            name:"Nil".to_string(),
            args: TypingContext{ bindings:vec![] }
        },
        XtorSig{ 
            xtor:Data,
            name:"Cons".to_string(),
            args:TypingContext{ 
                bindings:vec![
                    ContextBinding { var:"x".to_string(), chi:Chirality::Prd, ty:Ty::I64},
                    ContextBinding { var:"xs".to_string(), chi:Chirality::Prd, ty:Ty::Decl("ListInt".to_string())},
                ]
            }
        }
]);
let decl2 = DataDeclaration{
dat:Data,
        name:"List".to_string(),
        xtors:vec![
            XtorSig{
xtor:Data,
     name:"Nil".to_string(),
     args: TypingContext{ bindings:vec![] }
            },
        XtorSig{
xtor:Data,
     name:"Cons".to_string(),
     args:TypingContext{ 
bindings:vec![
             ContextBinding { var:"x".to_string(), chi:Chirality::Prd, ty:Ty::I64},
         ContextBinding { var:"xs".to_string(), chi:Chirality::Prd, ty:Ty::Decl("ListInt".to_string())},
]
     }
        }
        ]
};
assert_eq!(decl1,decl2);
```
