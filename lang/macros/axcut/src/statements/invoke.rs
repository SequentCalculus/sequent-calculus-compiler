use macro_utils::{expr_to_array, expr_to_string, expr_to_tuple, parse_args};
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
    let var = expr_to_tuple(&args[0]);
    let var_name = expr_to_string(&var[0], 0);
    let var_id = &var[1];
    let tag = expr_to_string(&args[1], 1);
    let ty = &args[2];
    let invoke_args = expr_to_array(&args[3], 3);
    quote! {
        axcut::syntax::statements::invoke::Invoke{
            var: axcut::syntax::names::Var {
                name:#var_name.to_string(),
                id: #var_id
            },
            tag: #tag.to_string(),
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
