Create a [`axcut::syntax::types::Ty`] from a string literal\
`int` will create [`axcut::syntax::types::Ty::I64`] anything else will create
[`axcut::syntax::types::Ty::Decl`]

```
use axcut::syntax::types::Ty;
use axcut_macros::ty;
let int1 = ty!("int");
let int2 = Ty::I64;
assert_eq!(int1, int2);
let list1 = ty!("ListInt");
let list2 = Ty::Decl("ListInt".to_string());
assert_eq!(list1, list2)
```
