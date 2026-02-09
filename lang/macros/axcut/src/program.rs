use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn prog(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Definitions", "Type Declarations"], &[]);
    let defs = expr_to_array(&args[0], 0);
    let types = expr_to_array(&args[1], 1);
    quote! {
        axcut::syntax::program::Prog{
            defs: ::std::vec::Vec::from([#(#defs),*]),
            types: ::std::vec::Vec::from([#(#types),*])
        }
    }
    .into()
}
