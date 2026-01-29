Create a [`core_lang::syntax::program::Prog`] ///

```
use core_lang::syntax::{
    context::TypingContext, declaration::TypeDeclaration, def::Def, program::Prog,
    statements::Exit, terms::XVar, types::Ty, Codata, Data,
};
use macros::prog;
use std::collections::HashSet;
let prog1 = prog!(
    [Def {
        name: "exit".to_string(),
        context: TypingContext::default(),
        body: Exit::exit(XVar::var("x", Ty::I64), Ty::I64),
        used_vars: HashSet::from(["x".to_string()])
    }],
    [TypeDeclaration {
        dat: Data,
        name: "Unit".to_string(),
        xtors: Vec::new()
    }],
    [TypeDeclaration {
        dat: Codata,
        name: "Void".to_string(),
        xtors: Vec::new()
    }]
);
let prog2 = Prog {
    defs: vec![Def {
        name: "exit".to_string(),
        context: TypingContext::default(),
        body: Exit::exit(XVar::var("x", Ty::I64), Ty::I64),
        used_vars: HashSet::from(["x".to_string()]),
    }],
    data_types: vec![TypeDeclaration {
        dat: Data,
        name: "Unit".to_string(),
        xtors: Vec::new(),
    }],
    codata_types: vec![TypeDeclaration {
        dat: Codata,
        name: "Void".to_string(),
        xtors: Vec::new(),
    }],
};
assert_eq!(prog1, prog2)
```
