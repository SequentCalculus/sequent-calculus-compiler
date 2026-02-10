use macro_utils::{expr_to_array, expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn call(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Call Label", "Call Arguments"], &[]);
    let label = expr_to_string(&args[0], 0);
    let call_args = expr_to_array(&args[1], 1);
    quote! {
        axcut::syntax::statements::call::Call{
            label:#label.to_string(),
            args: axcut::syntax::context::TypingContext{
                bindings: ::std::vec::Vec::from([
                    #(#call_args),*
                ])
            }
        }
    }
    .into()
}
