use macro_utils::{expr_to_array, parse_args, quote_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn letin(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        [
            "Variable",
            "Type",
            "Tag",
            "Arguments",
            "Next Statement",
            "Free Vars Next",
        ],
        &[
            (1, parse_str("axcut::syntax::types::Ty::I64").unwrap()),
            (5, parse_str("::std::option::Option::None").unwrap()),
        ],
    );
    let var = &args[0];
    let ty = &args[1];
    let tag = &args[2];
    let let_args = expr_to_array(&args[3], 3);
    let next = &args[4];
    let free_vars = quote_option(&args[5], |expr| {
        let free_vars = expr_to_array(expr, 5)
            .into_iter()
            .map(|expr| quote! {#expr})
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([ #(#free_vars),* ])
        }
    });

    quote! {
        axcut::syntax::statements::r#let::Let{
            var: #var,
            ty: #ty,
            tag: #tag,
            args: axcut::syntax::context::TypingContext{
                bindings: vec![
                    #(#let_args),*
                ]
            },
            next: ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#next)),
            free_vars_next: #free_vars
        }
    }
    .into()
}
