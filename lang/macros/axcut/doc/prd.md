Create [`axcut::syntax::context::Chirality::Prd`]

```
use axcut::syntax::context::Chirality;
use axcut_macros::prd;

let prd1 = prd!();
let prd2 = Chirality::Prd;
assert_eq!(prd1, prd2)
```
