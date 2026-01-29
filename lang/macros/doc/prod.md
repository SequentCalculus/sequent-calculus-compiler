Create a [`core_lang::syntax::terms::op::Op`] with
[`core_lang::syhntax::terms::op::BinOp::Prod`]

```
use core_lang::syntax::terms::{
    literal::Literal,
    op::{BinOp, Op},
    Term,
};
use macros::prod;
use std::rc::Rc;

let prod1 = prod!(Literal::new(1), Literal::new(2));
let prod2 = Op {
    fst: Rc::new(Term::from(Literal::new(1))),
    op: BinOp::Prod,
    snd: Rc::new(Term::from(Literal::new(2))),
};
assert_eq!(prod1, prod2)
```
