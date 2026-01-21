Create a [`core_lang::syntax::terms::op::Op`] with
[`core_lang::syhntax::terms::op::BinOp::Sub`]

```
use macros::sub;
use core_lang::syntax::{
    terms::{Term, literal::Literal, op::{Op,BinOp}}
};
use std::rc::Rc;

let sub1 = sub!(Literal::new(1),Literal::new(2));
let sub2 = Op{
   fst:Rc::new(Term::from(Literal::new(1))), 
   op:BinOp::Sub,
   snd:Rc::new(Term::from(Literal::new(2)))
};
assert_eq!(sub1,sub2)
```
