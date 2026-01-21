Create a [`core_lang::syntax::terms::ifc::IfC`]

```
use macros::ifc;
use core_lang::syntax::{types::Ty, statements::{exit::Exit,ifc::{IfSort,IfC}},terms::{Term, xvar::XVar,}};
use std::rc::Rc;

let if1 = ifc!(
    IfSort::Equal,
    XVar::var("x",Ty::I64),
    XVar::var("y",Ty::I64),
    Exit::exit(XVar::var("z",Ty::I64),Ty::I64),
    Exit::exit(XVar::var("w",Ty::I64),Ty::I64),
);
let if2 = IfC{
    sort:IfSort::Equal,
    fst:Rc::new(Term::from(XVar::var("x",Ty::I64))),
    snd:Some(Rc::new(Term::from(XVar::var("y",Ty::I64)))),
    thenc:Rc::new(Exit::exit(XVar::var("z",Ty::I64),Ty::I64).into()),
    elsec:Rc::new(Exit::exit(XVar::var("w",Ty::I64),Ty::I64).into())
    };
assert_eq!(if1,if2)
```
