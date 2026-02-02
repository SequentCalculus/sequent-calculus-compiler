use core_lang::syntax::Chirality;
use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn xcase(input: TokenStream, chi: Chirality) -> TokenStream {
    let prdcns = match chi {
        Chirality::Prd => quote! {core_lang::syntax::Prd},
        Chirality::Cns => quote! {core_lang::syntax::Cns},
    };
    let args = parse_args(
        input.into(),
        ["Case Clauses", "Type"],
        &[(1, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
    let clauses = expr_to_array(&args[0], 0);
    let ty = &args[1];
    quote! {
        core_lang::syntax::terms::xcase::XCase{
            prdcns: #prdcns,
            clauses: ::std::vec::Vec::from([
                #(#clauses),*
            ]),
            ty: #ty
        }
    }
    .into()
}
