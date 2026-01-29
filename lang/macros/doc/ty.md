Create a [`core_lang::syntax::types::Type`] from a string literal\
`int` will create [`core_lang::syntax::types::Type::I64`] anything else will
create [`core_lang::syntax::types::Type::Decl`]

```
use core_lang::syntax::types::Ty;
use macros::ty;
let int1 = ty!("int");
let int2 = Ty::I64;
assert_eq!(int1, int2);
let list1 = ty!("ListInt");
let list2 = Ty::Decl("ListInt".to_string());
assert_eq!(list1, list2)
```
