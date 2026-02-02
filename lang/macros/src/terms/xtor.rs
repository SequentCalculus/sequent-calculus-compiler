use crate::{
    arguments::arguments,
    context::typing_context,
    utils::{expr_to_str, parse_args},
};
use core_lang::syntax::Chirality;
use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

pub fn xtor(
    input: TokenStream,
    prdcns: Chirality,
    arg_fun: fn(&Expr, usize) -> proc_macro2::TokenStream,
) -> TokenStream {
    let (chi, xtor_desc) = match prdcns {
        Chirality::Prd => (quote! { core_lang::syntax::Prd}, "Ctor Name"),
        Chirality::Cns => (quote! { core_lang::syntax::Cns}, "Dtor Name"),
    };

    let args = parse_args(input, &[xtor_desc, "Argument list"], true);

    let xtor_name = expr_to_str(&args[0], 0);
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
