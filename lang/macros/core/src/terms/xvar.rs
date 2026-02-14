use core_lang::syntax::Chirality;
use macro_utils::parse_args;
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
        ["Variable Identifier", "Type"],
        &[(1, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
    let var_id = &args[0];
    let var_ty = &args[1];
    quote! {
        core_lang::syntax::terms::xvar::XVar{
            prdcns: #prdcns,
            var: #var_id,
            ty: #var_ty
        }
    }
    .into()
}
