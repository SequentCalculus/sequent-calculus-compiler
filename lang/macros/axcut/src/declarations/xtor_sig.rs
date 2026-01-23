use macro_utils::{expr_to_array, expr_to_str, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn xtor_sig(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Xtor Name", "Xtor Args"], &[]);
    let name = expr_to_str(&args[0]);
    let xtor_args = expr_to_array(&args[1]);
    quote! {
        axcut::syntax::declaration::XtorSig{
            name: #name.to_string(),
            args: axcut::syntax::context::TypingContext{
                bindings: ::std::vec::Vec::from([
                    #(#xtor_args),*
                ])
            },
        }
    }
    .into()
}
