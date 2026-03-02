use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn def(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Name", "Arguments", "Body"], &[]);
    let name = &args[0];
    let def_args = expr_to_array(&args[1], 1);
    let body = &args[2];
    quote! {
        axcut::syntax::def::Def{
            name:#name,
            context: axcut::syntax::context::TypingContext{
                bindings: vec![
                    #(#def_args),*
                ]
            },
            body: axcut::syntax::statements::Statement::from(#body),
        }
    }
    .into()
}
