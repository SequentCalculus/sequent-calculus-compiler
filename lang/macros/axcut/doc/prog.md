Create a [`axcut::syntax::program::Prog`]

```
use axcut::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    declaration::{TypeDeclaration, XtorSig},
    def::Def,
    program::Prog,
    statements::{Exit, Statement},
    types::Ty,
};
use axcut_macros::prog;
use std::collections::HashSet;

let prog1 = prog!(
    [Def {
        name: "main".to_string(),
        context: TypingContext { bindings: vec![] },
        body: Statement::from(Exit {
            var: "x".to_string()
        }),
        used_vars: HashSet::from(["x".to_string()])
    }],
    [TypeDeclaration {
        name: "ListInt".to_string(),
        xtors: vec![
            XtorSig {
                name: "Nil".to_string(),
                args: TypingContext { bindings: vec![] }
            },
            XtorSig {
                name: "Cons".to_string(),
                args: TypingContext {
                    bindings: vec![
                        ContextBinding {
                            var: "x".to_string(),
                            chi: Chirality::Ext,
                            ty: Ty::I64
                        },
                        ContextBinding {
                            var: "xs".to_string(),
                            chi: Chirality::Prd,
                            ty: Ty::Decl("ListInt".to_string())
                        }
                    ]
                }
            }
        ]
    }]
);
let prog2 = Prog {
    defs: vec![Def {
        name: "main".to_string(),
        context: TypingContext { bindings: vec![] },
        body: Statement::from(Exit {
            var: "x".to_string(),
        }),
        used_vars: HashSet::from(["x".to_string()]),
    }],
    types: vec![TypeDeclaration {
        name: "ListInt".to_string(),
        xtors: vec![
            XtorSig {
                name: "Nil".to_string(),
                args: TypingContext { bindings: vec![] },
            },
            XtorSig {
                name: "Cons".to_string(),
                args: TypingContext {
                    bindings: vec![
                        ContextBinding {
                            var: "x".to_string(),
                            chi: Chirality::Ext,
                            ty: Ty::I64,
                        },
                        ContextBinding {
                            var: "xs".to_string(),
                            chi: Chirality::Prd,
                            ty: Ty::Decl("ListInt".to_string()),
                        },
                    ],
                },
            },
        ],
    }],
};
assert_eq!(prog1, prog2)
```
