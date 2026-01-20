Create a [`core_lang::syntax::terms::Xtor`] with chirality
[`core_lang::syntax::terms::Prd`] (i.e. a constructor)

```
use macros::ctor;
use core_lang::syntax::{arguments::Arguments, types::Ty, terms::{XVar,Xtor}};
let ctor1 = ctor!("Cons",
    [XVar::var("x",Ty::I64),ctor!("Nil",[],Ty::Decl("ListInt".to_string()))],
    Ty::Decl("ListInt".to_string()));

let mut arguments = Arguments::default();
arguments.add_prod(XVar::var("x",Ty::I64));
arguments.add_prod(Xtor::ctor("Nil", Arguments::default(), Ty::Decl("ListInt".to_string()),));
let ctor2 = Xtor::ctor("Cons",arguments,Ty::Decl("ListInt".to_string()));
assert_eq!(ctor1,ctor2)
```
