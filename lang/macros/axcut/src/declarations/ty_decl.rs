use macro_utils::{expr_to_array, expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn ty_decl(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Type Name", "Type Xtors"], &[]);
    let name = expr_to_string(&args[0], 0);
    let xtors = expr_to_array(&args[1], 1);
    quote! {
        axcut::syntax::declaration::TypeDeclaration{
            name: #name.to_string(),
            xtors: ::std::vec::Vec::from([
                #(#xtors),*
            ])
        }
    }
    .into()
}
