Create a [`core_lang::syntax::terms::clause::Clause`]

```
use core_lang::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    statements::{Exit, Statement},
    terms::{clause::Clause, xvar::XVar},
    types::Ty,
    Prd,
};
use macros::clause;
use std::rc::Rc;

let clause1 = clause!(
    Prd,
    "apply",
    [ContextBinding {
        var: "x".to_string(),
        chi: Chirality::Prd,
        ty: Ty::I64
    }],
    Exit::exit(XVar::var("x", Ty::I64), Ty::I64)
);
let clause2 = Clause {
    prdcns: Prd,
    xtor: "apply".to_string(),
    context: TypingContext {
        bindings: vec![ContextBinding {
            var: "x".to_string(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        }],
    },
    body: Rc::new(Statement::from(Exit::exit(
        XVar::var("x", Ty::I64),
        Ty::I64,
    ))),
};
assert_eq!(clause1, clause2)
```
