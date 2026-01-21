Create a [`core_lang::syntax::terms::Xtor`] with chirality
[`core_lang::syntax::terms::Cns`] (i.e. a destructor)

```
use macros::dtor;
use core_lang::syntax::{arguments::Arguments, types::Ty, terms::{XVar,Xtor}};
let dtor1 = dtor!("apply",
    [XVar::var("x",Ty::I64)],
    Ty::Decl("FunI64I64".to_string()));

let mut arguments = Arguments::default();
arguments.add_prod(XVar::var("x",Ty::I64));
let dtor2 = Xtor::dtor("apply",arguments,Ty::Decl("FunI64I64".to_string()));
assert_eq!(dtor1,dtor2)
```
