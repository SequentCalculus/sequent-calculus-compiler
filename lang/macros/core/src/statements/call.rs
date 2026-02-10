use crate::{arguments::arguments, context::typing_context};
use macro_utils::{expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn unfocused_call(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Called Name", "Arguments", "Type"],
        &[(2, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
    let name = expr_to_string(&args[0], 0);
    let call_args = arguments(&args[1], 1);
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
    let args = parse_args(input.into(), ["Called Name", "Arguments"], &[]);
    let name = expr_to_string(&args[0], 0);
    let call_args = typing_context(&args[1], 1);
    quote! {
        core_lang::syntax::statements::call::FsCall{
            name: #name.to_string(),
            args: #call_args
        }
    }
    .into()
}
