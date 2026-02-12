use macro_utils::{expr_to_array, expr_to_string, expr_to_tuple, parse_args, quote_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn print_i64(input: TokenStream) -> TokenStream {
    print(input, false)
}

pub fn println_i64(input: TokenStream) -> TokenStream {
    print(input, true)
}

fn print(input: TokenStream, newline: bool) -> TokenStream {
    let newline = if newline {
        quote! {true}
    } else {
        quote! {false}
    };
    let args = parse_args(
        input.into(),
        ["Variable", "Next Statement", "Free Variables Next"],
        &[(2, parse_str("::std::option::Option::None").unwrap())],
    );

    let var = expr_to_tuple(&args[0]);
    let var_name = expr_to_string(&var[0], 0);
    let var_id = &var[1];
    let next = &args[1];
    let free_vars = quote_option(&args[2], |expr| {
        let free_vars = expr_to_array(expr, 2)
            .into_iter()
            .map(|expr| {
                let var = expr_to_tuple(&expr);
                let var_name = expr_to_string(&var[0], 2);
                let var_id = &var[1];

                quote! {
                    axcut::syntax::names::Var{
                        name:#var_name.to_string(),
                        id:#var_id
                    }
                }
            })
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([
                #(#free_vars),*
            ])
        }
    });
    quote! {
        axcut::syntax::statements::PrintI64{
            newline: #newline,
            var: axcut::syntax::names::Var{
                name:#var_name.to_string(),
                id:#var_id
            },
            next: ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#next)),
            free_vars_next: #free_vars
        }
    }
    .into()
}
