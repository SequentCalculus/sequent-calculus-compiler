use crate::{arguments::arguments, context::typing_context};
use core_lang::syntax::Chirality;
use macro_utils::{expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, parse_str};

pub fn xtor(
    input: TokenStream,
    prdcns: Chirality,
    arg_fun: fn(&Expr, usize) -> proc_macro2::TokenStream,
) -> TokenStream {
    let (chi, xtor_desc) = match prdcns {
        Chirality::Prd => (quote! { core_lang::syntax::Prd}, "Ctor Name"),
        Chirality::Cns => (quote! { core_lang::syntax::Cns}, "Dtor Name"),
    };

    let args = parse_args(
        input.into(),
        [xtor_desc, "Argument list", "Type"],
        &[(2, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );

    let xtor_name = expr_to_string(&args[0], 0);
    let xtor_args = arg_fun(&args[1], 1);
    let ty = &args[2];
    quote! {
        core_lang::syntax::terms::xtor::Xtor{
            prdcns: #chi,
            id: #xtor_name.to_string(),
            args: #xtor_args,
            ty: #ty
        }
    }
    .into()
}

pub fn unfocused_xtor(input: TokenStream, prdcns: Chirality) -> TokenStream {
    xtor(input, prdcns, arguments)
}

pub fn fs_xtor(input: TokenStream, prdcns: Chirality) -> TokenStream {
    xtor(input, prdcns, typing_context)
}
