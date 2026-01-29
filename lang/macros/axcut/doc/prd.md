Create [`axcut::syntax::context::Chirality::Prd`]

```
use axcut_macros::prd;
use axcut::syntax::context::Chirality;

let prd1 = prd!();
let prd2 = Chirality::Prd;
assert_eq!(prd1,prd2)
```
