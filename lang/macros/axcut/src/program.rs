use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn prog(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Definitions", "Type Declarations", "Max Used Id"],
        &[(2, parse_str("0").unwrap())],
    );
    let defs = expr_to_array(&args[0], 0);
    let types = expr_to_array(&args[1], 1);
    let max_id = &args[2];
    quote! {
        axcut::syntax::program::Prog{
            defs: ::std::vec::Vec::from([#(#defs),*]),
            types: ::std::vec::Vec::from([#(#types),*]),
            max_id:#max_id
        }
    }
    .into()
}
