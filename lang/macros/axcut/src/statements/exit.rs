use macro_utils::{expr_to_str, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn exit(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Variable"], &[]);
    let var = expr_to_str(&args[0], 0);
    quote! {
        axcut::syntax::statements::exit::Exit{
            var:#var.to_string()
        }
    }
    .into()
}
