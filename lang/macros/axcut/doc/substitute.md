Create a [`axcut::syntax::statements::substitute::Substitute`]

```
use axcut_macros::substitute;
use axcut::syntax::{
    statements::{exit::Exit, substitute::Substitute,Statement},
    context::{Chirality, ContextBinding},
    types::Ty
};
use std::rc::Rc;

let subst1 = substitute!([
    (ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},"x"),
    (ContextBinding{var:"a".to_string(),chi:Chirality::Cns,ty:Ty::Decl("Cont".to_string())},"a")
], Exit{var:"x".to_string()});
let subst2 = Substitute{
    rearrange: vec![
        (ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},"x".to_string()),
        (ContextBinding{var:"a".to_string(),chi:Chirality::Cns,ty:Ty::Decl("Cont".to_string())},"a".to_string())
    ],
    next:Rc::new(Statement::from(Exit{var:"x".to_string()}))
};
assert_eq!(subst1,subst2)
```
