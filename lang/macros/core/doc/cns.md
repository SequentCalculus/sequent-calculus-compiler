Create [`core_lang::syntax::context::Chirality::Cns`]

```
use core_lang::syntax::context::Chirality;
use core_macros::cns;

let cns1 = cns!();
let cns2 = Chirality::Cns;
assert_eq!(cns1, cns2)
```
