use macro_utils::{expr_to_array, expr_to_str, parse_args, quote_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn switch(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Variable", "Type", "Clauses", "Free Vars Clauses"],
        &[
            (1, parse_str("axcut::syntax::types::Ty::I64").unwrap()),
            (3, parse_str("::std::option::Option::None").unwrap()),
        ],
    );
    let var = expr_to_str(&args[0]);
    let ty = &args[1];
    let clauses = expr_to_array(&args[2]);
    let free_vars = quote_option(&args[3], |expr| {
        let free_arr = expr_to_array(expr)
            .into_iter()
            .map(|expr| quote! {#expr.to_string()})
            .collect::<Vec<_>>();
        quote! { ::std::collections::HashSet::from([ #(#free_arr),* ]) }
    });
    quote! {
        axcut::syntax::statements::switch::Switch{
            var:#var.to_string(),
            ty:#ty,
            clauses: ::std::vec::Vec::from([ #(#clauses),* ]),
            free_vars_clauses: #free_vars
        }
    }
    .into()
}
