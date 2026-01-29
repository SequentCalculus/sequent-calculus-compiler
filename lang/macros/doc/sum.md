Create a [`core_lang::syntax::terms::op::Op`] with
[`core_lang::syhntax::terms::op::BinOp::Sum`]

```
use core_lang::syntax::terms::{
    literal::Literal,
    op::{BinOp, Op},
    Term,
};
use macros::sum;
use std::rc::Rc;

let sum1 = sum!(Literal::new(1), Literal::new(2));
let sum2 = Op {
    fst: Rc::new(Term::from(Literal::new(1))),
    op: BinOp::Sum,
    snd: Rc::new(Term::from(Literal::new(2))),
};
assert_eq!(sum1, sum2)
```
