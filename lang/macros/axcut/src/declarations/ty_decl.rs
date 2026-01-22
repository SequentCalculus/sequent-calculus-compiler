use macro_utils::{expr_to_array, expr_to_str, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn ty_decl(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), &["Type Name", "Type Xtors"], false);
    let ty_name = expr_to_str(&args[0]);
    let ty_xtors = expr_to_array(&args[1]);
    quote! {
        axcut::syntax::declaration::TypeDeclaration{
            name: #ty_name.to_string(),
            xtors: ::std::vec::Vec::from([
                #(#ty_xtors),*
            ])
        }
    }
    .into()
}
