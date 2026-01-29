Create a [`axcut::syntax::statements::create::Create`]. `context`,
`free_vars_clauses` and `free_vars_next` are optional and default to `None` if
not provided. However, since they are parsed in order, if `context` is `None`
but `free_vars_clauses` is `Some`, `context` has to be provided (as `None`)

```
use axcut::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    statements::{clause::Clause, create::Create, exit::Exit, Statement},
    types::Ty,
};
use axcut_macros::create;
use std::{collections::HashSet, rc::Rc};

let create1 = create!(
    "x",
    Ty::Decl("FunIntInt".to_string()),
    [ContextBinding {
        var: "x".to_string(),
        chi: Chirality::Ext,
        ty: Ty::I64
    }],
    [Clause {
        xtor: "apply".to_string(),
        context: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: "y".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::I64
                },
                ContextBinding {
                    var: "a".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::Decl("Cont".to_string())
                }
            ]
        },
        body: Rc::new(Statement::from(Exit {
            var: "y".to_string()
        }))
    }],
    ["y", "a"],
    Exit {
        var: "x".to_string()
    },
    ["x"]
);
let create2 = Create {
    var: "x".to_string(),
    ty: Ty::Decl("FunIntInt".to_string()),
    context: Some(TypingContext {
        bindings: vec![ContextBinding {
            var: "x".to_string(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        }],
    }),
    clauses: vec![Clause {
        xtor: "apply".to_string(),
        context: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: "y".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: "a".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::Decl("Cont".to_string()),
                },
            ],
        },
        body: Rc::new(Statement::from(Exit {
            var: "y".to_string(),
        })),
    }],
    free_vars_clauses: Some(HashSet::from(["y".to_string(), "a".to_string()])),
    next: Rc::new(Statement::from(Exit {
        var: "x".to_string(),
    })),
    free_vars_next: Some(HashSet::from(["x".to_string()])),
};
assert_eq!(create1, create2);

let create1 = create!(
    "x",
    Ty::Decl("Cont".to_string()),
    [ContextBinding {
        var: "y".to_string(),
        chi: Chirality::Ext,
        ty: Ty::I64
    }],
    [Clause {
        xtor: "Cont".to_string(),
        context: TypingContext { bindings: vec![] },
        body: Rc::new(Statement::from(Exit {
            var: "y".to_string()
        }))
    }],
    Exit {
        var: "x".to_string()
    }
);
let create2 = Create {
    var: "x".to_string(),
    ty: Ty::Decl("Cont".to_string()),
    context: Some(TypingContext {
        bindings: vec![ContextBinding {
            var: "y".to_string(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        }],
    }),
    clauses: vec![Clause {
        xtor: "Cont".to_string(),
        context: TypingContext { bindings: vec![] },
        body: Rc::new(Statement::from(Exit {
            var: "y".to_string(),
        })),
    }],
    free_vars_clauses: None,
    next: Rc::new(Statement::from(Exit {
        var: "x".to_string(),
    })),
    free_vars_next: None,
};
assert_eq!(create1, create2);
```
