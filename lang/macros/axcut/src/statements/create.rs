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
            "Linearity Annotation",
            "Context",
            "Clauses",
            "Free Vars Clauses",
            "Next",
            "Free Vars Next",
        ],
        &[
            (3, parse_str("::std::option::Option::None").unwrap()),
            (2, parse_str("false").unwrap()),
            (5, parse_str("::std::option::Option::None").unwrap()),
            (7, parse_str("::std::option::Option::None").unwrap()),
        ],
    );

    let var = &args[0];
    let ty = &args[1];
    let linear = &args[2];
    let context = quote_option(&args[3], |expr| {
        let bindings = expr_to_array(expr, 3);
        quote! {
        axcut::syntax::context::TypingContext{
            bindings: ::std::vec::Vec::from([
                #(#bindings),*
            ])
        }}
    });
    let clauses = expr_to_array(&args[4], 4);
    let free_vars_clauses = quote_option(&args[5], |expr| {
        let free_vars_clauses = expr_to_array(expr, 5)
            .into_iter()
            .map(|expr| quote! {#expr})
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([
                #(#free_vars_clauses),*
            ])
        }
    });
    let next = &args[6];
    let free_vars_next = quote_option(&args[7], |expr| {
        let free_vars_next = expr_to_array(expr, 7)
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
            linear: #linear,
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
