Create a [`axcut::syntax::statements::exit::Exit`]

```
use axcut_macros::exit;
use axcut::syntax::statements::exit::Exit;

let exit1 = exit!("x");
let exit2 = Exit{
    var:"x".to_string()
};
assert_eq!(exit1,exit2)
```
