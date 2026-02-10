use core_lang::syntax::Chirality;
use macro_utils::{expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn xvar(input: TokenStream, chi: Chirality) -> TokenStream {
    let prdcns = match chi {
        Chirality::Prd => quote! {core_lang::syntax::terms::Prd},
        Chirality::Cns => quote! {core_lang::syntax::terms::Cns},
    };
    let args = parse_args(
        input.into(),
        ["Variable Name", "Type"],
        &[(1, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
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
