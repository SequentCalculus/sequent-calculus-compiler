use proc_macro::TokenStream;

pub(crate) mod context;
pub(crate) mod declarations;

#[doc=include_str!("../doc/bind.md")]
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    context::bind(input)
}

#[doc = include_str!("../doc/xtor_sig.md")]
#[proc_macro]
pub fn xtor_sig(input: TokenStream) -> TokenStream {
    declarations::xtor_sig(input)
}

#[doc=include_str!("../doc/ty_decl.md")]
#[proc_macro]
pub fn ty_decl(input: TokenStream) -> TokenStream {
    declarations::ty_decl(input)
}
