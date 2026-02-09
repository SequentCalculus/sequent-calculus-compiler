use macro_utils::{expr_to_array, expr_to_string, parse_args, quote_option};
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

    let var = expr_to_string(&args[0], 0);
    let next = &args[1];
    let free = quote_option(&args[2], |expr| {
        let expr_arr = expr_to_array(expr, 2)
            .into_iter()
            .map(|expr| quote! { #expr.to_string()})
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([
                #(#expr_arr),*
            ])
        }
    });
    quote! {
        axcut::syntax::statements::PrintI64{
            newline:#newline,
            var:#var.to_string(),
            next: ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#next)),
            free_vars_next: #free
        }
    }
    .into()
}
