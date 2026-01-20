use crate::utils::parse_args;
use proc_macro::TokenStream;
use quote::quote;

pub fn cut(input: TokenStream, term_ty: proc_macro2::TokenStream) -> TokenStream {
    let args = parse_args(input, &["Producer", "Consumer"], true);
    let prod = &args[0];
    let cons = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::statements::Cut{
            producer: ::std::rc::Rc::new(#term_ty::from(#prod)),
            consumer: ::std::rc::Rc::new(#term_ty::from(#cons)),
            ty:#ty
        }
    }
    .into()
}

pub fn unfocused_cut(input: TokenStream) -> TokenStream {
    cut(input, quote! {core_lang::syntax::terms::Term})
}

pub fn fs_cut(input: TokenStream) -> TokenStream {
    cut(input, quote! {core_lang::syntax::terms::FsTerm})
}
