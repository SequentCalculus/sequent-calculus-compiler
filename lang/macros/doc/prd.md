Create [`core_lang::syntax::context::Chirality::Prd`]

```
use core_lang::syntax::context::Chirality;
use macros::prd;

let cns1 = prd!();
let cns2 = Chirality::Prd;
assert_eq!(cns1, cns2)
```
