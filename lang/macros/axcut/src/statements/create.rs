use macro_utils::{expr_to_array, expr_to_str, is_none};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Token, parse::Parser, punctuated::Punctuated};

pub fn create(input: TokenStream) -> TokenStream {
    let args = parse_create_args(input);
    let var = &args[0];
    let ty = &args[1];
    let clauses = &args[2];
    let next = &args[3];
    let context = &args[4];
    let free_vars_clauses = &args[5];
    let free_vars_next = &args[6];

    quote! {
        axcut::syntax::statements::create::Create{
            var:#var,
            ty:#ty,
            clauses:#clauses,
            next:#next,
            context:#context,
            free_vars_clauses:#free_vars_clauses,
            free_vars_next:#free_vars_next
        }
    }
    .into()
}
pub fn parse_create_args(input: TokenStream) -> Vec<proc_macro2::TokenStream> {
    let mut parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(input.into())
        .expect("Macro arguments could not be parsed")
        .into_iter();
    let var = expr_to_str(&parsed.next().expect("Please provide create variable"));
    let ty = parsed.next().expect("Please provide create type");
    let clauses = expr_to_array(&parsed.next().expect("Please provide create clauses"));
    let next = parsed.next().expect("Please provide next statement");
    let context = if let Some(expr) = parsed.next() {
        let exprs = expr_to_array(&expr);
        quote! { ::std::option::Option::Some(axcut::syntax::context::TypingContext{
            bindings: ::std::vec::Vec::from([
                #(#exprs),*
            ])
        }) }
    } else {
        quote!(::std::option::Option::None)
    };
    let free_vars_clauses = if let Some(expr) = parsed.next() {
        let exprs = expr_to_array(&expr)
            .into_iter()
            .map(|expr| quote! {#expr.to_string()})
            .collect::<Vec<_>>();
        quote! { ::std::option::Option::Some(::std::collections::HashSet::from([
              #(#exprs),*
            ]))
        }
    } else {
        quote! {::std::option::Option::None}
    };
    let free_vars_next = if let Some(expr) = parsed.next() {
        if is_none(&expr) {
            quote! { ::std::option::Option::None }
        } else {
            let exprs = expr_to_array(&expr)
                .into_iter()
                .map(|expr| quote! {#expr.to_string()})
                .collect::<Vec<_>>();
            quote! {
                ::std::option::Option::Some(::std::collections::HashSet::from([
                    #(#exprs),*
                ]))
            }
        }
    } else {
        quote! { ::std::option::Option::None }
    };

    vec![
        quote! { #var.to_string() },
        quote! { #ty },
        quote! { ::std::vec::Vec::from([ #(#clauses),* ]) },
        quote! { ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#next)) },
        context,
        free_vars_clauses,
        free_vars_next,
    ]
}
