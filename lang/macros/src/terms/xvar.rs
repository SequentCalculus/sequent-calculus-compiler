use crate::utils::{expr_to_string, parse_args};
use core_lang::syntax::Chirality;
use proc_macro::TokenStream;
use quote::quote;

pub fn xvar(input: TokenStream, chi: Chirality) -> TokenStream {
    let prdcns = match chi {
        Chirality::Prd => quote! {core_lang::syntax::terms::Prd},
        Chirality::Cns => quote! {core_lang::syntax::terms::Cns},
    };
    let args = parse_args(input, &["Variable Name"], true);
    let var_name = expr_to_string(&args[0], 0);
    let var_ty = &args[1];
    quote! {
        core_lang::syntax::terms::xvar::XVar{
            prdcns: #prdcns,
            var: #var_name.to_string(),
            ty: #var_ty
        }
    }
    .into()
}
