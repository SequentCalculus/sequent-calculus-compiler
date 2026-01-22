use macro_utils::{expr_to_array, expr_to_str, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn invoke(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        &["Invoked variable", "Invoked tag", "Invoke Arguments"],
        true,
    );
    let var = expr_to_str(&args[0]);
    let tag = expr_to_str(&args[1]);
    let invoke_args = expr_to_array(&args[2]);
    let ty = &args[3];
    quote! {
        axcut::syntax::statements::invoke::Invoke{
            var: #var.to_string(),
            tag: #tag.to_string(),
            args: axcut::syntax::context::TypingContext {
                bindings: ::std::vec::Vec::from([
                    #(#invoke_args),*
                ])
            },
            ty:#ty
        }
    }
    .into()
}
