Create a [`core_lang::syntax::declaration::CodataDeclaration`]

```
use core_lang::syntax::{
    context::{Chirality, ContextBinding, TypingContext},
    declaration::{Codata, CodataDeclaration, XtorSig},
    types::Ty,
};
use macros::codata;
let codata1 = codata!(
    "FunIntInt",
    [XtorSig {
        xtor: Codata,
        name: "apply".to_string(),
        args: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: "x".to_string(),
                    chi: Chirality::Prd,
                    ty: Ty::I64
                },
                ContextBinding {
                    var: "a".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::I64
                },
            ]
        }
    }]
);
let codata2 = CodataDeclaration {
    dat: Codata,
    name: "FunIntInt".to_string(),
    xtors: vec![XtorSig {
        xtor: Codata,
        name: "apply".to_string(),
        args: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: "x".to_string(),
                    chi: Chirality::Prd,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: "a".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::I64,
                },
            ],
        },
    }],
};
assert_eq!(codata1, codata2);
```
