use macro_utils::{expr_to_string, expr_to_tuple, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn exit(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Variable"], &[]);
    let var = expr_to_tuple(&args[0]);
    let var_name = expr_to_string(&var[0], 0);
    let var_id = &var[1];
    quote! {
        axcut::syntax::statements::exit::Exit{
            var: axcut::syntax::names::Var{
                name:#var_name.to_string(),
                id: #var_id
            }
        }
    }
    .into()
}
