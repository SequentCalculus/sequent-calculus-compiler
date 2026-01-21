Create a [`core_lang::syntax::terms::op::Op`] with
[`core_lang::syhntax::terms::op::BinOp::Div`]

```
use macros::div;
use core_lang::syntax::{
    terms::{Term, literal::Literal, op::{Op,BinOp}}
};
use std::rc::Rc;

let div1= div!(Literal::new(1),Literal::new(2));
let div2 = Op{
   fst:Rc::new(Term::from(Literal::new(1))), 
   op:BinOp::Div,
   snd:Rc::new(Term::from(Literal::new(2)))
};
assert_eq!(div1,div2);
```
