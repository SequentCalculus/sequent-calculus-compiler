Create a [`core_lang::syntax::terms::clause::Clause`]

```
use macros::clause;
use core_lang::syntax::{ Prd,
    context::{Chirality, ContextBinding,TypingContext},
    terms::{xvar::XVar,clause::Clause},
    types::Ty,
    statements::{Statement,Exit},
};
use std::rc::Rc;

let clause1 = clause!(Prd,"apply",
    [ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64}],
    Exit::exit(XVar::var("x",Ty::I64),Ty::I64)
);
let clause2 = Clause{
    prdcns:Prd,
    xtor:"apply".to_string(),
    context:TypingContext{ bindings: vec![
        ContextBinding{ var:"x".to_string(), chi:Chirality::Prd, ty:Ty::I64 }
    ] },
    body:Rc::new(Statement::from(Exit::exit(XVar::var("x",Ty::I64),Ty::I64))),
};
assert_eq!(clause1,clause2)
```
