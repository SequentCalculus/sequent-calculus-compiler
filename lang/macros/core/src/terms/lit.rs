use macro_utils::parse_args;
use proc_macro::TokenStream;
use quote::quote;

pub fn lit(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Literal"], &[]);
    let lit = &args[0];
    quote! {core_lang::syntax::terms::literal::Literal::new(#lit)}.into()
}
