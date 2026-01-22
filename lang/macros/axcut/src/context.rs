use macro_utils::expr_to_str;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Token, parse::Parser, punctuated::Punctuated};

pub fn bind(input: TokenStream) -> TokenStream {
    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(input.into())
        .expect("Macro arguments could not be parsed")
        .into_iter()
        .collect::<Vec<_>>();
    let bind_var = expr_to_str(parsed.get(0).expect("Please provided binding variable"));
    let bind_chi;
    let bind_ty;
    if parsed.len() == 1 {
        bind_chi = quote! { axcut::syntax::context::Chirality::Ext };
        bind_ty = quote! { axcut::syntax::types::Ty::I64 };
    } else {
        let chi = parsed.get(1).expect("Please provide binding chirality");
        bind_chi = quote! { #chi };
        let ty = parsed.get(2).expect("Please provide binding ty");
        bind_ty = quote! { #ty };
    }
    quote! {
        axcut::syntax::context::ContextBinding{
            var: #bind_var.to_string(),
            chi: #bind_chi,
            ty: #bind_ty
        }
    }
    .into()
}
