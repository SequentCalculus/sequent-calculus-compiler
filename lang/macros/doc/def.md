Create a [`core_lang::syntax::def::Def`]

```
use macros::def;
use core_lang::syntax::{
    def::Def,
    types::Ty,
    context::{Chirality, ContextBinding, TypingContext},
    statements::{Statement, Call},
    arguments::{Argument,Arguments},
    terms::{xvar::XVar, Term}
};
use std::collections::HashSet;
let def1 = def!(
    "print",
    [ContextBinding{ var:"x".to_string(), chi: Chirality::Prd, ty: Ty::I64 } ],
    Call {
        name:"print_i64".to_string(),
        args: Arguments { entries: vec![Argument::from(Term::from(XVar::var("x",Ty::I64)))] },
        ty:Ty::I64
    }, ["a","x"]);
let def2 = Def {
    name:"print".to_string(),
    context: TypingContext{
        bindings: vec![ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64}]
    },
    body:Statement::from(Call {
        name:"print_i64".to_string(),
        args: Arguments { entries: vec![Argument::from(Term::from(XVar::var("x",Ty::I64)))] },
        ty:Ty::I64
    }),
    used_vars: HashSet::from(["x".to_string(),"a".to_string()])};
assert_eq!(def1,def2)
```
