Create [`axcut::syntax::context::Chirality::Ext`]

```
use axcut::syntax::context::Chirality;
use axcut_macros::ext;
let ext1 = ext!();
let ext2 = Chirality::Ext;
assert_eq!(ext1, ext2)
```
