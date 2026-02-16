use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn ty_decl(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Type Name", "Type Xtors"], &[]);
    let name = &args[0];
    let xtors = expr_to_array(&args[1], 1);
    quote! {
        axcut::syntax::declaration::TypeDeclaration{
            name: #name,
            xtors: ::std::vec::Vec::from([
                #(#xtors),*
            ])
        }
    }
    .into()
}
