Create a [`core_lang::syntax::statements::ifc::FsIfC`]

```
use macros::fs_ifc;
use core_lang::syntax::{
    types::Ty, 
    statements::{exit::FsExit,ifc::{IfSort, FsIfC},FsStatement}
};
use std::rc::Rc;

let if1 = fs_ifc!(IfSort::Equal,"x","y",FsExit::exit("x"),FsExit::exit("y"));
let if2 = FsIfC{
    sort:IfSort::Equal,
    fst: "x".to_string(),
    snd: Some("y".to_string()),
    thenc:Rc::new(FsStatement::from(FsExit::exit("x"))),
    elsec:Rc::new(FsStatement::from(FsExit::exit("y")))
};
assert_eq!(if1,if2)
```
