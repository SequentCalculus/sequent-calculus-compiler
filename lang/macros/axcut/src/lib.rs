use proc_macro::TokenStream;

pub(crate) mod context;
pub(crate) mod declarations;
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

// Statements
#[doc=include_str!("../doc/substitute.md")]
#[proc_macro]
pub fn substitute(input: TokenStream) -> TokenStream {
    statements::substitute(input)
}

#[doc=include_str!("../doc/invoke.md")]
#[proc_macro]
pub fn invoke(input: TokenStream) -> TokenStream {
    statements::invoke(input)
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

// Declarations
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
