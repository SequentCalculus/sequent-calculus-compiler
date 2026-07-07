use crate::arguments::arguments;
use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn unfocused_call(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Called Name", "Type Arguments", "Arguments", "Type"],
        &[
            (1, parse_str("[]").unwrap()),
            (3, parse_str("core_lang::syntax::types::Ty::I64").unwrap()),
        ],
    );
    let name = &args[0];
    let type_args = expr_to_array(&args[1], 1);
    let call_args = arguments(&args[2], 1);
    let ty = &args[3];
    quote! {
        core_lang::syntax::statements::call::Call{
            name: #name,
            type_args: core_lang::syntax::types::TypeArgs {
            args: ::std::vec![ #(#type_args),* ],
        },
            args: #call_args,
            ty: #ty
        }
    }
    .into()
}

pub fn fs_call(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Called Name", "Type Arguments", "Arguments"],
        &[(1, parse_str("[]").unwrap())],
    );
    let name = &args[0];
    let type_args = expr_to_array(&args[1], 1);
    let call_args = expr_to_array(&args[2], 1);
    quote! {
        core_lang::syntax::statements::call::FsCall{
            name: #name,
            type_args: core_lang::syntax::types::TypeArgs {
                args: ::std::vec![ #(#type_args),* ],
            },
            args: core_lang::syntax::TypingContext::from(::std::vec![ #(#call_args),* ])
        }
    }
    .into()
}
