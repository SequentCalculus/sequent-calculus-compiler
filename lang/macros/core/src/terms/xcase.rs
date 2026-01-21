use crate::utils::{expr_to_array, parse_args};
use core_lang::syntax::Chirality;
use proc_macro::TokenStream;
use quote::quote;

pub fn xcase(input: TokenStream, chi: Chirality) -> TokenStream {
    let prdcns = match chi {
        Chirality::Prd => quote! {core_lang::syntax::Prd},
        Chirality::Cns => quote! {core_lang::syntax::Cns},
    };
    let args = parse_args(input, &["Case Clauses"], true);
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
