Create a [`axcut::syntax::statements::create::Create`].
`context,fvree_vars_clauses` and `free_vars_next` are optional and can be left
out. In this case they default to `None`. However, the order of these arguments
needs to stay in tact, so skipping `context` but providing `free_vars_clauses`
requires arguments `None,[...]` For example
`create!(...,Exit{var:"x".to_string()})` will set `context:None`,
`free_vars_clauses:None` and `free_vars_next:None`, but
`create!(...Exit{var:"x".to_string()},["x"],None,["y"])` will set
`context: HashSet::from(["x".to_string()])`, `free_vars_clauses:None` and
`free_vars_next:HashSet::from(["y".to_string()])`

```
use axcut_macros::create;
use axcut::syntax::{
    statements::{Statement, clause::Clause, create::Create,exit::Exit},
    context::{TypingContext,ContextBinding,Chirality},
    types::Ty
};
use std::{collections::HashSet, rc::Rc};

let create1 = create!(
    "x",
    Ty::Decl("FunIntInt".to_string()),
    [Clause{
        xtor:"apply".to_string(),
        context:TypingContext{
            bindings:vec![
                ContextBinding{var:"y".to_string(),chi:Chirality::Ext,ty:Ty::I64},
                ContextBinding{var:"a".to_string(),chi:Chirality::Cns,ty:Ty::Decl("Cont".to_string())}
            ]
        },
        body:Rc::new(Statement::from(Exit{var:"y".to_string()}))
    }],
    Exit{var:"x".to_string()},
    [ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64}],
    ["y","a"],["x"]
);
let create2 = Create{
    var:"x".to_string(),
    ty:Ty::Decl("FunIntInt".to_string()),
    clauses: vec![
        Clause{
            xtor:"apply".to_string(),
            context:TypingContext{
                bindings:vec![
                    ContextBinding{var:"y".to_string(),chi:Chirality::Ext,ty:Ty::I64},
                    ContextBinding{var:"a".to_string(),chi:Chirality::Cns,ty:Ty::Decl("Cont".to_string())}
                ]
            },
            body:Rc::new(Statement::from(Exit{var:"y".to_string()}))
        }
    ],
    next: Rc::new(Statement::from(Exit{var:"x".to_string()})),
    context: Some(TypingContext{
        bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Ext,ty:Ty::I64},
        ]
    }),
    free_vars_clauses: Some(HashSet::from(["y".to_string(),"a".to_string()])),
    free_vars_next:Some(HashSet::from(["x".to_string()]))
};
assert_eq!(create1,create2);
```
