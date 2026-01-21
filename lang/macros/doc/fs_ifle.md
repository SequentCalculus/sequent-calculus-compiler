Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
[`core_lang::syntax::statements::ifc::IfSort::LessOrEqual`]. If only one
comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)

```
use macros::fs_ifle;
use core_lang::syntax::{
    statements::{FsStatement, ifc::{IfSort, IfC},FsExit},
    types::Ty,
    terms::{FsTerm, Literal}
};
use std::rc::Rc;

let if1 = fs_ifle!("x","y",FsExit::exit("x"),FsExit::exit("y"));
let if2 = IfC{
    sort: IfSort::LessOrEqual,
    fst: "x".to_string(),
    snd: Some("y".to_string()),
    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
    elsec: Rc::new(FsStatement::from(FsExit::exit("y")))
};
assert_eq!(if1,if2);

let if1 = fs_ifle!("x",FsExit::exit("x"),FsExit::exit("y"));
let if2 = IfC{
    sort: IfSort::LessOrEqual,
    fst: "x".to_string(),
    snd: None,
    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
};
assert_eq!(if1,if2);
```
