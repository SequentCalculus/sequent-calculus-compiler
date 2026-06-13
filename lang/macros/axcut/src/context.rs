use macro_utils::parse_args;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn bind(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Variable", "Chirality", "Type", "Quantity"],
        &[
            (
                1,
                parse_str("axcut::syntax::context::Chirality::Ext").unwrap(),
            ),
            (2, parse_str("axcut::syntax::types::Ty::I64").unwrap()),
            (
                3,
                parse_str("axcut::syntax::context::Quantity::Omega").unwrap(),
            ),
        ],
    );
    let var = &args[0];
    let chi = &args[1];
    let ty = &args[2];
    let quantity = &args[3];
    quote! {
        axcut::syntax::context::ContextBinding{
            var: #var,
            chi: #chi,
            quantity: #quantity,
            ty: #ty
        }
    }
    .into()
}
