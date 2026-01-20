use crate::{
    arguments::arguments,
    utils::{expr_to_str, parse_args},
};
use proc_macro::TokenStream;
use quote::quote;

pub fn call(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Called Name", "Arguments"], true);
    let name = expr_to_str(&args[0]);
    let call_args = arguments(&args[1]);
    let call_ty = &args[2];
    quote! {
        core_lang::syntax::statements::call::Call{
            name: #name.to_string(),
            args: #call_args,
            ty:#call_ty
        }
    }
    .into()
}
