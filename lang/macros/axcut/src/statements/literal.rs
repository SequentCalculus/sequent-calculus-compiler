use macro_utils::{expr_to_array, expr_to_string, parse_args, quote_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn lit(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Literal", "Variable", "Next", "Free Vars Next"],
        &[(3, parse_str("::std::option::Option::None").unwrap())],
    );
    let lit = &args[0];
    let var = expr_to_string(&args[1], 1);
    let next = &args[2];
    let free = quote_option(&args[3], |expr| {
        let free_arr = expr_to_array(expr, 3)
            .into_iter()
            .map(|expr| quote! {#expr.to_string()})
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([
                #(#free_arr),*
            ])
        }
    });
    quote! {
        axcut::syntax::statements::literal::Literal{
            lit:#lit,
            var:#var.to_string(),
            next: ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#next)),
            free_vars_next: #free
        }
    }
    .into()
}
