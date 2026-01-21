Create a [`core_lang::syntax::terms::xcase::XCase`] with chirality
[`core_lang::syntax::Cns`], i.e. a case expression If the continuation type is
not specified, it defaults to [`core_lang::syntax::types::Ty::I64`]

```
use macros::case;
use core_lang::syntax::{
    Cns,
    statements::{Exit,Statement},
    context::{Chirality, ContextBinding,TypingContext},
    terms::{clause::Clause,xvar::XVar, xcase::XCase},
    types::Ty,
};
use std::rc::Rc;

let case1 = case!([
    Clause{
        prdcns:Cns,
        xtor:"Nil".to_string(),
        context:TypingContext::default(),
        body:Rc::new(Statement::from(Exit::exit(XVar::var("x",Ty::I64),Ty::I64)))
    },
    Clause{
        prdcns:Cns,
        xtor:"Cons".to_string(),
        context:TypingContext{
                bindings:vec![
                    ContextBinding{
                        var:"x".to_string(),
                        chi:Chirality::Prd,
                        ty:Ty::I64
                    },
                    ContextBinding{
                        var:"xs".to_string(),
                        chi:Chirality::Prd,
                        ty:Ty::Decl("ListInt".to_string())
                    },
                    ContextBinding{
                        var:"a".to_string(),
                        chi:Chirality::Cns,
                        ty:Ty::Decl("ListInt".to_string())
                    }
                ]
            },
            body:Rc::new(Statement::from(Exit::exit(XVar::var("x",Ty::I64),Ty::I64)))
        }
    ],Ty::Decl("ListInt".to_string()));

let case2 = XCase{
    prdcns:Cns,
    clauses: vec![
        Clause{
            prdcns:Cns,
            xtor:"Nil".to_string(),
            context:TypingContext::default(),
            body:Rc::new(Statement::from(Exit::exit(XVar::var("x",Ty::I64),Ty::I64)))
        },
        Clause{
            prdcns:Cns,
            xtor:"Cons".to_string(),
            context:TypingContext{
                bindings:vec![
                    ContextBinding{
                        var:"x".to_string(),
                        chi:Chirality::Prd,
                        ty:Ty::I64
                    },
                    ContextBinding{
                        var:"xs".to_string(),
                        chi:Chirality::Prd,
                        ty:Ty::Decl("ListInt".to_string())
                    },
                    ContextBinding{
                        var:"a".to_string(),
                        chi:Chirality::Cns,
                        ty:Ty::Decl("ListInt".to_string())
                    }
                ]
            },
            body:Rc::new(Statement::from(Exit::exit(XVar::var("x",Ty::I64),Ty::I64)))
        }
    ],
    ty:Ty::Decl("ListInt".to_string())
};
assert_eq!(case1,case2)
```
