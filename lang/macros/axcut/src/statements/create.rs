use macro_utils::{expr_to_array, parse_args, quote_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn create(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        [
            "Variable",
            "Type",
            "Context",
            "Clauses",
            "Free Vars Clauses",
            "Next",
            "Free Vars Next",
        ],
        &[
            (2, parse_str("::std::option::Option::None").unwrap()),
            (4, parse_str("::std::option::Option::None").unwrap()),
            (6, parse_str("::std::option::Option::None").unwrap()),
        ],
    );

    let var = &args[0];
    let ty = &args[1];
    let context = quote_option(&args[2], |expr| {
        let bindings = expr_to_array(expr, 2);
        quote! {
        axcut::syntax::context::TypingContext{
            bindings: ::std::vec::Vec::from([
                #(#bindings),*
            ])
        }}
    });
    let clauses = expr_to_array(&args[3], 3);
    let free_vars_clauses = quote_option(&args[4], |expr| {
        let free_vars_clauses = expr_to_array(expr, 4)
            .into_iter()
            .map(|expr| quote! {#expr})
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([
                #(#free_vars_clauses),*
            ])
        }
    });
    let next = &args[5];
    let free_vars_next = quote_option(&args[6], |expr| {
        let free_vars_next = expr_to_array(expr, 6)
            .into_iter()
            .map(|expr| quote! {#expr})
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([
                #(#free_vars_next),*
            ])
        }
    });

    quote! {
        axcut::syntax::statements::create::Create{
            var: #var,
            ty: #ty,
            context: #context,
            clauses: std::vec::Vec::from([
                #(#clauses),*
            ]),
            next: ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#next)),
            free_vars_clauses: #free_vars_clauses,
            free_vars_next: #free_vars_next,
        }
    }
    .into()
}
