use macro_utils::{expr_to_array, parse_args, quote_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn switch(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        [
            "Variable",
            "Type",
            "Linearity Annotation",
            "Clauses",
            "Free Vars Clauses",
        ],
        &[
            (1, parse_str("axcut::syntax::types::Ty::I64").unwrap()),
            (2, parse_str("false").unwrap()),
            (4, parse_str("::std::option::Option::None").unwrap()),
        ],
    );
    let var = &args[0];
    let ty = &args[1];
    let linear = &args[2];
    let clauses = expr_to_array(&args[3], 3);
    let free_vars = quote_option(&args[4], |expr| {
        let free_vars = expr_to_array(expr, 4)
            .into_iter()
            .map(|expr| quote! {#expr})
            .collect::<Vec<_>>();
        quote! { ::std::collections::HashSet::from([ #(#free_vars),* ]) }
    });
    quote! {
        axcut::syntax::statements::switch::Switch{
            var: #var,
            ty: #ty,
            linear: #linear,
            clauses: ::std::vec::Vec::from([ #(#clauses),* ]),
            free_vars_clauses: #free_vars
        }
    }
    .into()
}
