Create a [`core_lang::syntax::terms::op::Op`] with
[`core_lang::syhntax::terms::op::BinOp::Prod`]

```
use macros::prod;
use core_lang::syntax::{
    terms::{Term, literal::Literal, op::{Op,BinOp}}
};
use std::rc::Rc;

let prod1 = prod!(Literal::new(1),Literal::new(2));
let prod2 = Op{
   fst:Rc::new(Term::from(Literal::new(1))), 
   op:BinOp::Prod,
   snd:Rc::new(Term::from(Literal::new(2)))
};
assert_eq!(prod1,prod2)
```
