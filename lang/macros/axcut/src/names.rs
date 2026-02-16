use macro_utils::{expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn id(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Name", "Id"],
        &[(1, parse_str("0").unwrap())],
    );
    let name = expr_to_string(&args[0], 0);
    let id = &args[1];
    quote! {
        axcut::syntax::names::Ident{
            name:#name.to_string(),
            id:#id
        }
    }
    .into()
}
