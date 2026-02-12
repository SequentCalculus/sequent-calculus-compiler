use macro_utils::{expr_to_string, expr_to_tuple, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn bind(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Variable", "Variable Id", "Chirality", "Type"],
        &[
            (
                2,
                parse_str("axcut::syntax::context::Chirality::Ext").unwrap(),
            ),
            (3, parse_str("axcut::syntax::types::Ty::I64").unwrap()),
        ],
    );
    let var_name = expr_to_string(&args[0], 0);
    let var_id = &args[1];
    let chi = &args[2];
    let ty = &args[3];
    quote! {
        axcut::syntax::context::ContextBinding{
            var: axcut::syntax::names::Var {
                name: #var_name.to_string(),
                id: #var_id
            },
            chi: #chi,
            ty: #ty
        }
    }
    .into()
}
