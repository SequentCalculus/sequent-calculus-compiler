use macro_utils::{expr_to_array, expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, parse_str};

pub fn bind(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Context Variable", "Context Chirality", "Type"],
        &[(2, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
    let var = expr_to_string(&args[0], 0);
    let chi = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::context::ContextBinding{
            var: #var.to_string(),
            chi: #chi,
            ty: #ty
        }
    }
    .into()
}

pub fn typing_context(arg: &Expr, number_of_args: usize) -> proc_macro2::TokenStream {
    let args = expr_to_array(arg, number_of_args);
    quote! {
        core_lang::syntax::context::TypingContext { bindings: ::std::vec::Vec::from([
            #(#args),*
        ]) }
    }
}
