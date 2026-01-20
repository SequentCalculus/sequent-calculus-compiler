use core_lang::syntax::Chirality;
use proc_macro::TokenStream;

pub(crate) mod arguments;
pub(crate) mod context;
pub(crate) mod definitions;
pub(crate) mod prog;
pub(crate) mod statements;
pub(crate) mod terms;
pub(crate) mod types;
mod utils;
use terms::{xcase, xtor, xvar};

#[doc = include_str!("../doc/ty.md")]
#[proc_macro]
pub fn ty(input: TokenStream) -> TokenStream {
    types::ty(input)
}

#[doc = include_str!("../doc/var.md")]
#[proc_macro]
pub fn var(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Prd)
}

#[doc = include_str!("../doc/covar.md")]
#[proc_macro]
pub fn covar(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Cns)
}

#[doc = include_str!("../doc/cut.md")]
#[proc_macro]
pub fn cut(input: TokenStream) -> TokenStream {
    statements::cut(input)
}

#[doc = include_str!("../doc/ctor.md")]
#[proc_macro]
pub fn ctor(input: TokenStream) -> TokenStream {
    xtor(input, Chirality::Prd)
}

#[doc = include_str!("../doc/dtor.md")]
#[proc_macro]
pub fn dtor(input: TokenStream) -> TokenStream {
    xtor(input, Chirality::Cns)
}

#[doc = include_str!("../doc/ifc.md")]
#[proc_macro]
pub fn ifc(input: TokenStream) -> TokenStream {
    statements::ifc(input)
}

#[doc = include_str!("../doc/ifcz.md")]
#[proc_macro]
pub fn ifcz(input: TokenStream) -> TokenStream {
    statements::ifcz(input)
}

#[doc = include_str!("../doc/call.md")]
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    statements::call(input)
}

#[doc = include_str!("../doc/exit.md")]
#[proc_macro]
pub fn exit(input: TokenStream) -> TokenStream {
    statements::exit(input)
}

#[doc = include_str!("../doc/bind.md")]
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    context::bind(input)
}

#[doc = include_str!("../doc/def.md")]
#[proc_macro]
pub fn def(input: TokenStream) -> TokenStream {
    definitions::def(input)
}

#[doc = include_str!("../doc/prog.md")]
#[proc_macro]
pub fn prog(input: TokenStream) -> TokenStream {
    prog::prog(input)
}

#[doc=include_str!("../doc/op.md")]
#[proc_macro]
pub fn op(input: TokenStream) -> TokenStream {
    statements::op(input)
}

#[doc = include_str!("../doc/clause.md")]
#[proc_macro]
pub fn clause(input: TokenStream) -> TokenStream {
    terms::clause(input)
}

#[doc=include_str!("../doc/case.md")]
#[proc_macro]
pub fn case(input: TokenStream) -> TokenStream {
    xcase(input, Chirality::Cns)
}

#[doc=include_str!("../doc/cocase.md")]
#[proc_macro]
pub fn cocase(input: TokenStream) -> TokenStream {
    xcase(input, Chirality::Prd)
}

#[doc=include_str!("../doc/mu.md")]
#[proc_macro]
pub fn mu(input: TokenStream) -> TokenStream {
    terms::xmu(input, Chirality::Prd)
}

#[doc=include_str!("../doc/mutilde.md")]
#[proc_macro]
pub fn mutilde(input: TokenStream) -> TokenStream {
    terms::xmu(input, Chirality::Cns)
}
