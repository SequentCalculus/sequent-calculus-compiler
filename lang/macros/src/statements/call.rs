use crate::{
    arguments::arguments,
    context::typing_context,
    utils::{expr_to_str, parse_args},
};
use proc_macro::TokenStream;
use quote::quote;

pub fn unfocused_call(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Called Name", "Arguments"], true);
    let name = expr_to_str(&args[0]);
    let call_args = arguments(&args[1]);
    let ty = &args[2];
    quote! {
        core_lang::syntax::statements::call::Call{
            name: #name.to_string(),
            args: #call_args,
            ty: #ty
        }
    }
    .into()
}

pub fn fs_call(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Called Name", "Arguments"], false);
    let name = expr_to_str(&args[0]);
    let call_args = typing_context(&args[1]);
    quote! {
        core_lang::syntax::statements::call::FsCall{
            name: #name.to_string(),
            args: #call_args
        }
    }
    .into()
}
