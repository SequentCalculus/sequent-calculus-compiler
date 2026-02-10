use macro_utils::{expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn bind(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Variable", "Chirality", "Type"],
        &[
            (
                1,
                parse_str("axcut::syntax::context::Chirality::Ext").unwrap(),
            ),
            (2, parse_str("axcut::syntax::types::Ty::I64").unwrap()),
        ],
    );
    let var = expr_to_string(&args[0], 0);
    let chi = &args[1];
    let ty = &args[2];
    quote! {
        axcut::syntax::context::ContextBinding{
            var: #var.to_string(),
            chi: #chi,
            ty: #ty
        }
    }
    .into()
}
