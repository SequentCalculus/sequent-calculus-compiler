use macro_utils::{expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn ty(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Type Name"], &[]);
    let ty = expr_to_string(&args[0], 0);
    if ty == "int" {
        quote! {core_lang::syntax::types::Ty::I64}
    } else {
        quote! {core_lang::syntax::types::Ty::Decl(#ty.to_string())}
    }
    .into()
}
