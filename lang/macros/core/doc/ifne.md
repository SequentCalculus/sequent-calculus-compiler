Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
[`core_lang::syntax::statements::ifc::IfSort::NotEqual`]. If only one comparison
argument is provided, default to using zero (i.e. `IfC.snd == None`)

```
use core_lang::syntax::{
    statements::{
        ifc::{IfC, IfSort},
        Exit, Statement,
    },
    terms::{literal::Literal, Term},
    types::Ty,
};
use core_macros::ifne;
use std::rc::Rc;

let if1 = ifne!(
    Literal::new(1),
    Literal::new(1),
    Exit::exit(Literal::new(1), Ty::I64),
    Exit::exit(Literal::new(2), Ty::I64)
);
let if2 = IfC {
    sort: IfSort::NotEqual,
    fst: Rc::new(Term::from(Literal::new(1))),
    snd: Some(Rc::new(Term::from(Literal::new(1)))),
    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
};
assert_eq!(if1, if2);

let if1 = ifne!(
    Literal::new(1),
    Exit::exit(Literal::new(1), Ty::I64),
    Exit::exit(Literal::new(2), Ty::I64)
);
let if2 = IfC {
    sort: IfSort::NotEqual,
    fst: Rc::new(Term::from(Literal::new(1))),
    snd: None,
    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
};
assert_eq!(if1, if2);
```
