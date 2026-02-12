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
        ["Variable Name", "Variable Id", "Type"],
        &[(2, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
    let var_name = expr_to_string(&args[0], 0);
    let var_id = &args[1];
    let var_ty = &args[2];
    quote! {
        core_lang::syntax::terms::xvar::XVar{
            prdcns: #prdcns,
            var: core_lang::syntax::names::Var {
                name: #var_name.to_string(),
                id:#var_id
            },
            ty: #var_ty
        }
    }
    .into()
}
