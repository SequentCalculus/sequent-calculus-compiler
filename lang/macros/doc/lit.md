Create a [`core_lang::syntax::terms::literal::Literal`]

```
use core_lang::syntax::terms::literal::Literal;
use macros::lit;

let lit1 = lit!(1);
let lit2 = Literal { lit: 1 };
assert_eq!(lit1, lit2)
```
