use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn invoke(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        [
            "Invoked variable",
            "Invoked tag",
            "Type",
            "Invoke Arguments",
        ],
        &[(3, parse_str("axcut::syntax::types::Ty::I64").unwrap())],
    );
    let var = &args[0];
    let tag = &args[1];
    let ty = &args[2];
    let invoke_args = expr_to_array(&args[3], 3);
    quote! {
        axcut::syntax::statements::invoke::Invoke{
            var: #var,
            tag: #tag,
            args: axcut::syntax::context::TypingContext {
                bindings: ::std::vec::Vec::from([
                    #(#invoke_args),*
                ])
            },
            ty: #ty
        }
    }
    .into()
}
