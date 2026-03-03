use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn prog(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        [
            "Definitions",
            "Data Declarations",
            "Codata Declarations",
            "Max Used Identifier Id",
        ],
        &[(3, parse_str("0").unwrap())],
    );
    let def_list = expr_to_array(&args[0], 0);
    let data_list = expr_to_array(&args[1], 1);
    let codata_list = expr_to_array(&args[2], 2);
    let max_id = &args[3];
    quote! {
        core_lang::syntax::program::Prog{
            defs: ::std::vec::Vec::from([
                #(#def_list),*
            ]),
            data_types: ::std::vec::Vec::from([
                #(#data_list),*
            ]),
            codata_types: ::std::vec::Vec::from([
                #(#codata_list),*
            ]),
            max_id: #max_id
        }
    }
    .into()
}
