Create a [`core_lang::syntax::statements::FsExit`]

```
use macros::fs_exit;
use core_lang::syntax::statements::FsExit;
let exit1 = fs_exit!("x");
let exit2 = FsExit{
    var:"x".to_string()
};
assert_eq!(exit1,exit2);
```
