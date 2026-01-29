Create [`axcut::syntax::context::Chirality::Ext`]

```
use axcut_macros::ext;
use axcut::syntax::context::Chirality;
let ext1 = ext!();
let ext2 = Chirality::Ext;
assert_eq!(ext1,ext2)
```
