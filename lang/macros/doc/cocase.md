Create a [`core_lang::syntax::terms::xcase::XCase`] with chirality
[`core_lang::syntax::Cns`] i.e. a cocase / new expression if the return type is
not specified it defaults to [`core_lang::syntax::types::Ty::I64`]

```
use macros::cocase;
use core_lang::syntax::{
    Cns,Prd,
    terms::{xvar::XVar,xcase::XCase,clause::Clause},
    statements::exit::Exit,
    context::{ContextBinding,Chirality,TypingContext},
    types::Ty
};
use std::rc::Rc;

let cocase1 =
cocase!([
    Clause{
        prdcns:Prd,
        xtor:"apply".to_string(),
        context: TypingContext{ bindings:vec![
            ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64},
            ContextBinding{var:"a".to_string(),chi:Chirality::Cns,ty:Ty::I64}
        ]},
        body:Rc::new(Exit::exit(XVar::var("x",Ty::I64),Ty::I64)) }
    ],Ty::Decl("FunI64I64".to_string()));
let cocase2 = XCase{
    prdcns:Prd,
    clauses:vec![
        Clause{
            prdcns:Prd,
            xtor:"apply".to_string(),
            context:TypingContext{
                bindings:vec![
                    ContextBinding{ var:"x".to_string(), chi:Chirality::Prd, ty:Ty::I64},
                    ContextBinding{ var:"a".to_string(), chi:Chirality::Cns, ty:Ty::I64},
                ]
            },
            body:Rc::new(Exit::exit(XVar::var("x",Ty::I64),Ty::I64))

        }
    ],
    ty:Ty::Decl("FunI64I64".to_string()),

};
assert_eq!(cocase1,cocase2)
```
