use crate::utils::{expr_to_str, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn bind(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Context Variable", "Context Chirality"], true);
    let var = expr_to_str(&args[0]);
    let chi = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::context::ContextBinding{
            var: #var.to_string(),
            chi: #chi,
            ty: #ty
        }
    }
    .into()
}
