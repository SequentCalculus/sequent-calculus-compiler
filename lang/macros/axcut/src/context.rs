use macro_utils::expr_to_str;
use macro_utils::parse_args;
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
    let bind_var = expr_to_str(&args[0], 0);
    let bind_chi = &args[1];
    let bind_ty = &args[2];
    quote! {
        axcut::syntax::context::ContextBinding{
            var: #bind_var.to_string(),
            chi: #bind_chi,
            ty: #bind_ty
        }
    }
    .into()
}
