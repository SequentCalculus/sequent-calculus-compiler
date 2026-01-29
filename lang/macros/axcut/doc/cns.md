Create [`axcut::syntax::context::Chirality::Cns`]

```
use axcut::syntax::context::Chirality;
use axcut_macros::cns;
let cns1 = cns!();
let cns2 = Chirality::Cns;
assert_eq!(cns1, cns2);
```
