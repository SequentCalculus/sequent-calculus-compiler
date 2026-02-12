use macro_utils::{expr_to_array, expr_to_string, expr_to_tuple, parse_args, quote_option};
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
    let var = expr_to_tuple(&args[1]);
    let var_name = expr_to_string(&var[0], 0);
    let var_id = &var[1];
    let next = &args[2];
    let free_vars = quote_option(&args[3], |expr| {
        let free_vars = expr_to_array(expr, 3)
            .into_iter()
            .map(|expr| quote! {#expr.to_string()})
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([
                #(#free_vars),*
            ])
        }
    });
    quote! {
        axcut::syntax::statements::literal::Literal{
            lit: #lit,
            var: axcut::syntax::names::Var {
                name:#var_name.to_string(),
                id:#var_id
            },
            next: ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#next)),
            free_vars_next: #free_vars
        }
    }
    .into()
}
