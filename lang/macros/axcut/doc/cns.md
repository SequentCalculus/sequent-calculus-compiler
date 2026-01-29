Create [`axcut::syntax::context::Chirality::Cns`]

```
use axcut_macros::cns;
use axcut::syntax::context::Chirality;
let cns1 = cns!();
let cns2 = Chirality::Cns;
assert_eq!(cns1,cns2);
```
