Create a [`core_lang::syntax::context::ContextBinding`] If no type is provided,
it defaults to [`core_lang::syntax::types::Ty`]

```
use macros::bind;
use core_lang::syntax::{types::Ty, context::{ContextBinding,Chirality}};
let bnd1 = bind!("x",Chirality::Prd);
let bnd2 = bind!("x",Chirality::Prd,Ty::I64);
let bnd3 = ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64};
assert_eq!(bnd1,bnd2);
assert_eq!(bnd2,bnd3);
```
