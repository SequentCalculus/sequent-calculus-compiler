Create a [`core_lang::syntax::def::Def`]

```
use core_lang::syntax::{
    arguments::{Argument, Arguments},
    context::{Chirality, ContextBinding, TypingContext},
    def::Def,
    statements::{Call, Statement},
    terms::{xvar::XVar, Term},
    types::Ty,
};
use macros::def;
use std::collections::HashSet;
let def1 = def!(
    "print",
    [ContextBinding {
        var: "x".to_string(),
        chi: Chirality::Prd,
        ty: Ty::I64
    }],
    Call {
        name: "print_i64".to_string(),
        args: Arguments {
            entries: vec![Argument::from(Term::from(XVar::var("x", Ty::I64)))]
        },
        ty: Ty::I64
    },
    ["a", "x"]
);
let def2 = Def {
    name: "print".to_string(),
    context: TypingContext {
        bindings: vec![ContextBinding {
            var: "x".to_string(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        }],
    },
    body: Statement::from(Call {
        name: "print_i64".to_string(),
        args: Arguments {
            entries: vec![Argument::from(Term::from(XVar::var("x", Ty::I64)))],
        },
        ty: Ty::I64,
    }),
    used_vars: HashSet::from(["x".to_string(), "a".to_string()]),
};
assert_eq!(def1, def2)
```
