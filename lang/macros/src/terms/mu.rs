use crate::utils::{expr_to_string, parse_args};
use core_lang::syntax::context::Chirality;
use proc_macro::TokenStream;
use quote::quote;

pub fn xmu(
    input: TokenStream,
    chi: Chirality,
    statement_kind: proc_macro2::TokenStream,
) -> TokenStream {
    let prdcns = match chi {
        Chirality::Prd => quote! { core_lang::syntax::Prd },
        Chirality::Cns => quote! { core_lang::syntax::Cns },
    };
    let args = parse_args(input, &["Bound Variable", "Bound Statement"], true);
    let var = expr_to_string(&args[0], 0);
    let stmt = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::terms::mu::Mu{
            prdcns: #prdcns,
            variable: #var.to_string(),
            statement: ::std::rc::Rc::new(#statement_kind::from(#stmt)),
            ty: #ty
        }
    }
    .into()
}

pub fn unfocused_xmu(input: TokenStream, chi: Chirality) -> TokenStream {
    xmu(
        input,
        chi,
        quote! {core_lang::syntax::statements::Statement},
    )
}

pub fn fs_xmu(input: TokenStream, chi: Chirality) -> TokenStream {
    xmu(
        input,
        chi,
        quote! {core_lang::syntax::statements::FsStatement},
    )
}
