Create a [`core_lang::syntax::terms::op::Op`] with
[`core_lang::syhntax::terms::op::BinOp::Rem`]

```
use macros::rem;
use core_lang::syntax::{
    terms::{Term, literal::Literal, op::{Op,BinOp}}
};
use std::rc::Rc;

let rem1 = rem!(Literal::new(1),Literal::new(2));
let rem2 = Op{
   fst:Rc::new(Term::from(Literal::new(1))), 
   op:BinOp::Rem,
   snd:Rc::new(Term::from(Literal::new(2)))
};
assert_eq!(rem1,rem2)
```
