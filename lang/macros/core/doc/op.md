Create a [`core_lang::syntax::terms::op::Op`]

```
use macros::op;
use core_lang::syntax::terms::{literal::Literal, Term, op::{BinOp,Op}};
use std::rc::Rc;

let op1 = op!(Literal::new(1),BinOp::Sum, Literal::new(2));
let op2 = Op{
    fst:Rc::new(Term::from(Literal::new(1))),
    op:BinOp::Sum,
    snd:Rc::new(Term::from(Literal::new(2)))
};
assert_eq!(op1,op2)
```
