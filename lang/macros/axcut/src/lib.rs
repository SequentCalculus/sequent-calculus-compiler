use proc_macro::TokenStream;
use quote::quote;

pub(crate) mod context;
pub(crate) mod declarations;
pub(crate) mod program;
pub(crate) mod statements;
pub(crate) mod types;

#[doc=include_str!("../doc/ty.md")]
#[proc_macro]
pub fn ty(input: TokenStream) -> TokenStream {
    types::ty(input)
}

#[doc=include_str!("../doc/bind.md")]
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    context::bind(input)
}

#[doc=include_str!("../doc/prd.md")]
#[proc_macro]
pub fn prd(_: TokenStream) -> TokenStream {
    quote! {axcut::syntax::context::Chirality::Prd}.into()
}

#[doc=include_str!("../doc/cns.md")]
#[proc_macro]
pub fn cns(_: TokenStream) -> TokenStream {
    quote! {axcut::syntax::context::Chirality::Cns}.into()
}

#[doc=include_str!("../doc/ext.md")]
#[proc_macro]
pub fn ext(_: TokenStream) -> TokenStream {
    quote! {axcut::syntax::context::Chirality::Ext}.into()
}

// Statements
#[doc=include_str!("../doc/substitute.md")]
#[proc_macro]
pub fn substitute(input: TokenStream) -> TokenStream {
    statements::substitute(input)
}

#[doc=include_str!("../doc/let.md")]
#[proc_macro]
pub fn letin(input: TokenStream) -> TokenStream {
    statements::letin(input)
}

#[doc=include_str!("../doc/switch.md")]
#[proc_macro]
pub fn switch(input: TokenStream) -> TokenStream {
    statements::switch(input)
}

#[doc=include_str!("../doc/invoke.md")]
#[proc_macro]
pub fn invoke(input: TokenStream) -> TokenStream {
    statements::invoke(input)
}

#[doc=include_str!("../doc/create.md")]
#[proc_macro]
pub fn create(input: TokenStream) -> TokenStream {
    statements::create(input)
}

#[doc=include_str!("../doc/literal.md")]
#[proc_macro]
pub fn lit(input: TokenStream) -> TokenStream {
    statements::lit(input)
}

#[doc=include_str!("../doc/call.md")]
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    statements::call(input)
}

#[doc=include_str!("../doc/print.md")]
#[proc_macro]
pub fn print_i64(input: TokenStream) -> TokenStream {
    statements::print_i64(input)
}

#[doc=include_str!("../doc/println.md")]
#[proc_macro]
pub fn println_i64(input: TokenStream) -> TokenStream {
    statements::println_i64(input)
}

#[doc=include_str!("../doc/exit.md")]
#[proc_macro]
pub fn exit(input: TokenStream) -> TokenStream {
    statements::exit(input)
}

#[doc=include_str!("../doc/clause.md")]
#[proc_macro]
pub fn clause(input: TokenStream) -> TokenStream {
    statements::clause(input)
}

#[doc=include_str!("../doc/div.md")]
#[proc_macro]
pub fn div(input: TokenStream) -> TokenStream {
    statements::div(input)
}

#[doc=include_str!("../doc/prod.md")]
#[proc_macro]
pub fn prod(input: TokenStream) -> TokenStream {
    statements::prod(input)
}

#[doc=include_str!("../doc/rem.md")]
#[proc_macro]
pub fn rem(input: TokenStream) -> TokenStream {
    statements::rem(input)
}

#[doc=include_str!("../doc/sum.md")]
#[proc_macro]
pub fn sum(input: TokenStream) -> TokenStream {
    statements::sum(input)
}

#[doc=include_str!("../doc/sub.md")]
#[proc_macro]
pub fn sub(input: TokenStream) -> TokenStream {
    statements::sub(input)
}

#[doc = include_str!("../doc/ife.md")]
#[proc_macro]
pub fn ife(input: TokenStream) -> TokenStream {
    statements::ife(input)
}

#[doc = include_str!("../doc/ifne.md")]
#[proc_macro]
pub fn ifne(input: TokenStream) -> TokenStream {
    statements::ifne(input)
}

#[doc = include_str!("../doc/ifl.md")]
#[proc_macro]
pub fn ifl(input: TokenStream) -> TokenStream {
    statements::ifl(input)
}

#[doc = include_str!("../doc/ifle.md")]
#[proc_macro]
pub fn ifle(input: TokenStream) -> TokenStream {
    statements::ifle(input)
}

#[doc = include_str!("../doc/ifg.md")]
#[proc_macro]
pub fn ifg(input: TokenStream) -> TokenStream {
    statements::ifg(input)
}

#[doc = include_str!("../doc/ifge.md")]
#[proc_macro]
pub fn ifge(input: TokenStream) -> TokenStream {
    statements::ifge(input)
}

// Declarations
#[doc = include_str!("../doc/xtor_sig.md")]
#[proc_macro]
pub fn xtor_sig(input: TokenStream) -> TokenStream {
    declarations::xtor_sig(input)
}

#[doc=include_str!("../doc/def.md")]
#[proc_macro]
pub fn def(input: TokenStream) -> TokenStream {
    declarations::def(input)
}

#[doc=include_str!("../doc/ty_decl.md")]
#[proc_macro]
pub fn ty_decl(input: TokenStream) -> TokenStream {
    declarations::ty_decl(input)
}

#[doc=include_str!("../doc/prog.md")]
#[proc_macro]
pub fn prog(input: TokenStream) -> TokenStream {
    program::prog(input)
}
