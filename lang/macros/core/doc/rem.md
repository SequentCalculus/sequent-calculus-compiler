Create a [`core_lang::syntax::terms::op::Op`] with
[`core_lang::syntax::terms::op::BinOp::Rem`]

```
use core_lang::syntax::terms::{
    literal::Literal,
    op::{BinOp, Op},
    Term,
};
use core_macros::rem;
use std::rc::Rc;

let rem1 = rem!(Literal::new(1), Literal::new(2));
let rem2 = Op {
    fst: Rc::new(Term::from(Literal::new(1))),
    op: BinOp::Rem,
    snd: Rc::new(Term::from(Literal::new(2))),
};
assert_eq!(rem1, rem2)
```
