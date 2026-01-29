Create a [`core_lang::syntax::terms::literal::Lit`]

```
use macros::lit;
use core_lang::syntax::terms::literal::Literal;

let lit1 = lit!(1);
let lit2 = Literal {
    lit: 1
};
assert_eq!(lit1,lit2)
```
