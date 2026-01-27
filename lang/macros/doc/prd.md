Create [`core_lang::syntax::context::Chirality::Prd`]

```
use macros::prd;
use core_lang::syntax::context::Chirality;

let cns1 = prd!();
let cns2 = Chirality::Prd;
assert_eq!(cns1,cns2)
```
