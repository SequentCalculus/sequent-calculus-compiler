use macro_utils::{expr_to_array, expr_to_string, is_none, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn def(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Name", "Arguments", "Body", "Body Used Vars"],
        &[(3, parse_str("::std::option::Option::None").unwrap())],
    );
    let name = expr_to_string(&args[0], 0);
    let def_args = expr_to_array(&args[1], 1);
    let body = &args[2];
    let used_vars = if is_none(&args[3]) {
        quote! { ::std::collections::HashSet::new() }
    } else {
        let used_arr = expr_to_array(&args[3], 3)
            .into_iter()
            .map(|expr| quote! {#expr.to_string()})
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([
                #(#used_arr),*
            ])
        }
    };
    quote! {
        axcut::syntax::def::Def{
            name:#name.to_string(),
            context: axcut::syntax::context::TypingContext{
                bindings: vec![
                    #(#def_args),*
                ]
            },
            body: axcut::syntax::statements::Statement::from(#body),
            used_vars: #used_vars
        }
    }
    .into()
}
