use macro_utils::parse_args;
use proc_macro::TokenStream;
use quote::quote;

pub fn exit(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Variable"], &[]);
    let var = &args[0];
    quote! {
        axcut::syntax::statements::exit::Exit{
            var: #var
        }
    }
    .into()
}
