use crate::{arguments::arguments, context::typing_context};
use macro_utils::parse_args;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn unfocused_call(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Called Name", "Arguments", "Type"],
        &[(2, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
    let name = &args[0];
    let call_args = arguments(&args[1], 1);
    let ty = &args[2];
    quote! {
        core_lang::syntax::statements::call::Call{
            name: #name,
            args: #call_args,
            ty: #ty
        }
    }
    .into()
}

pub fn fs_call(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Called Name", "Arguments"], &[]);
    let name = &args[0];
    let call_args = typing_context(&args[1], 1);
    quote! {
        core_lang::syntax::statements::call::FsCall{
            name: #name,
            args: #call_args
        }
    }
    .into()
}
