Create a [`core_lang::syntax::statements::ifc::FsIfC`]

```
use macros::fs_ifcz;
use core_lang::syntax::{
    statements::{ ifc::{IfSort, FsIfC},exit::FsExit,FsStatement},
    types::Ty
};
use std::rc::Rc;

let if1 = fs_ifcz!(IfSort::Equal,"x",FsExit::exit("x"),FsExit::exit("x"));
let if2 = FsIfC{
    sort:IfSort::Equal,
    fst: "x".to_string(),
    snd: None,
    thenc:Rc::new(FsStatement::from(FsExit::exit("x"))),
    elsec:Rc::new(FsStatement::from(FsExit::exit("x")))
};
assert_eq!(if1,if2)
```
