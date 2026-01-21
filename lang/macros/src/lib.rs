use core_lang::syntax::Chirality;
use proc_macro::TokenStream;

pub(crate) mod arguments;
pub(crate) mod context;
pub(crate) mod declarations;
pub(crate) mod prog;
pub(crate) mod statements;
pub(crate) mod terms;
pub(crate) mod types;
mod utils;
use terms::{fs_xtor, unfocused_xtor, xcase, xvar};

#[doc = include_str!("../doc/ty.md")]
#[proc_macro]
pub fn ty(input: TokenStream) -> TokenStream {
    types::ty(input)
}

#[doc = include_str!("../doc/bind.md")]
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    context::bind(input)
}

// Terms

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

#[doc = include_str!("../doc/ctor.md")]
#[proc_macro]
pub fn ctor(input: TokenStream) -> TokenStream {
    unfocused_xtor(input, Chirality::Prd)
}

#[doc = include_str!("../doc/dtor.md")]
#[proc_macro]
pub fn dtor(input: TokenStream) -> TokenStream {
    unfocused_xtor(input, Chirality::Cns)
}

#[doc = include_str!("../doc/fs_ctor.md")]
#[proc_macro]
pub fn fs_ctor(input: TokenStream) -> TokenStream {
    fs_xtor(input, Chirality::Prd)
}

#[doc = include_str!("../doc/fs_dtor.md")]
#[proc_macro]
pub fn fs_dtor(input: TokenStream) -> TokenStream {
    fs_xtor(input, Chirality::Cns)
}

#[doc = include_str!("../doc/clause.md")]
#[proc_macro]
pub fn clause(input: TokenStream) -> TokenStream {
    terms::unfocused_clause(input)
}

#[doc = include_str!("../doc/fs_clause.md")]
#[proc_macro]
pub fn fs_clause(input: TokenStream) -> TokenStream {
    terms::fs_clause(input)
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
    terms::unfocused_xmu(input, Chirality::Prd)
}

#[doc=include_str!("../doc/mutilde.md")]
#[proc_macro]
pub fn mutilde(input: TokenStream) -> TokenStream {
    terms::unfocused_xmu(input, Chirality::Cns)
}

#[doc=include_str!("../doc/fs_mu.md")]
#[proc_macro]
pub fn fs_mu(input: TokenStream) -> TokenStream {
    terms::fs_xmu(input, Chirality::Prd)
}

#[doc=include_str!("../doc/fs_mutilde.md")]
#[proc_macro]
pub fn fs_mutilde(input: TokenStream) -> TokenStream {
    terms::fs_xmu(input, Chirality::Cns)
}

// Statements

#[doc = include_str!("../doc/cut.md")]
#[proc_macro]
pub fn cut(input: TokenStream) -> TokenStream {
    statements::unfocused_cut(input)
}

#[doc = include_str!("../doc/fs_cut.md")]
#[proc_macro]
pub fn fs_cut(input: TokenStream) -> TokenStream {
    statements::fs_cut(input)
}

#[doc = include_str!("../doc/ife.md")]
#[proc_macro]
pub fn ife(input: TokenStream) -> TokenStream {
    statements::unfocused_ife(input)
}

#[doc = include_str!("../doc/ifne.md")]
#[proc_macro]
pub fn ifne(input: TokenStream) -> TokenStream {
    statements::unfocused_ifne(input)
}

#[doc = include_str!("../doc/ifl.md")]
#[proc_macro]
pub fn ifl(input: TokenStream) -> TokenStream {
    statements::unfocused_ifl(input)
}

#[doc = include_str!("../doc/ifle.md")]
#[proc_macro]
pub fn ifle(input: TokenStream) -> TokenStream {
    statements::unfocused_ifle(input)
}

#[doc = include_str!("../doc/ifg.md")]
#[proc_macro]
pub fn ifg(input: TokenStream) -> TokenStream {
    statements::unfocused_ifg(input)
}

#[doc = include_str!("../doc/ifge.md")]
#[proc_macro]
pub fn ifge(input: TokenStream) -> TokenStream {
    statements::unfocused_ifge(input)
}

#[doc = include_str!("../doc/fs_ife.md")]
#[proc_macro]
pub fn fs_ife(input: TokenStream) -> TokenStream {
    statements::fs_ife(input)
}

#[doc = include_str!("../doc/fs_ifne.md")]
#[proc_macro]
pub fn fs_ifne(input: TokenStream) -> TokenStream {
    statements::fs_ifne(input)
}

#[doc = include_str!("../doc/fs_ifl.md")]
#[proc_macro]
pub fn fs_ifl(input: TokenStream) -> TokenStream {
    statements::fs_ifl(input)
}

#[doc = include_str!("../doc/fs_ifle.md")]
#[proc_macro]
pub fn fs_ifle(input: TokenStream) -> TokenStream {
    statements::fs_ifle(input)
}

#[doc = include_str!("../doc/fs_ifg.md")]
#[proc_macro]
pub fn fs_ifg(input: TokenStream) -> TokenStream {
    statements::fs_ifg(input)
}

#[doc = include_str!("../doc/fs_ifge.md")]
#[proc_macro]
pub fn fs_ifge(input: TokenStream) -> TokenStream {
    statements::fs_ifge(input)
}

#[doc = include_str!("../doc/call.md")]
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    statements::unfocused_call(input)
}

#[doc = include_str!("../doc/fs_call.md")]
#[proc_macro]
pub fn fs_call(input: TokenStream) -> TokenStream {
    statements::fs_call(input)
}

#[doc = include_str!("../doc/exit.md")]
#[proc_macro]
pub fn exit(input: TokenStream) -> TokenStream {
    statements::exit(input)
}

#[doc=include_str!("../doc/op.md")]
#[proc_macro]
pub fn op(input: TokenStream) -> TokenStream {
    statements::unfocused_op(input)
}

#[doc=include_str!("../doc/fs_op.md")]
#[proc_macro]
pub fn fs_op(input: TokenStream) -> TokenStream {
    statements::fs_op(input)
}

// Declarations

#[doc = include_str!("../doc/def.md")]
#[proc_macro]
pub fn def(input: TokenStream) -> TokenStream {
    declarations::unfocused_def(input)
}

#[doc = include_str!("../doc/fs_def.md")]
#[proc_macro]
pub fn fs_def(input: TokenStream) -> TokenStream {
    declarations::fs_def(input)
}

#[doc = include_str!("../doc/data.md")]
#[proc_macro]
pub fn data(input: TokenStream) -> TokenStream {
    declarations::data(input)
}

#[doc=include_str!("../doc/codata.md")]
#[proc_macro]
pub fn codata(input: TokenStream) -> TokenStream {
    declarations::codata(input)
}

#[doc=include_str!("../doc/ctor_sig.md")]
#[proc_macro]
pub fn ctor_sig(input: TokenStream) -> TokenStream {
    declarations::ctor_sig(input)
}

#[doc=include_str!("../doc/dtor_sig.md")]
#[proc_macro]
pub fn dtor_sig(input: TokenStream) -> TokenStream {
    declarations::dtor_sig(input)
}

#[doc = include_str!("../doc/prog.md")]
#[proc_macro]
pub fn prog(input: TokenStream) -> TokenStream {
    prog::prog(input)
}
