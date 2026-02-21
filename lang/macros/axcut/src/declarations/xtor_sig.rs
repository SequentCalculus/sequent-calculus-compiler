use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn xtor_sig(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Xtor Name", "Xtor Args"], &[]);
    let name = &args[0];
    let xtor_args = expr_to_array(&args[1], 1);
    quote! {
        axcut::syntax::declaration::XtorSig{
            name: #name,
            args: axcut::syntax::context::TypingContext{
                bindings: ::std::vec::Vec::from([
                    #(#xtor_args),*
                ])
            },
        }
    }
    .into()
}
