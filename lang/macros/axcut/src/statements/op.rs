use axcut::syntax::statements::op::BinOp;
use macro_utils::{expr_to_array, expr_to_str};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Token, parse::Parser, punctuated::Punctuated};

pub fn div(input: TokenStream) -> TokenStream {
    op(input, BinOp::Div)
}
pub fn prod(input: TokenStream) -> TokenStream {
    op(input, BinOp::Prod)
}
pub fn rem(input: TokenStream) -> TokenStream {
    op(input, BinOp::Rem)
}
pub fn sub(input: TokenStream) -> TokenStream {
    op(input, BinOp::Sub)
}
pub fn sum(input: TokenStream) -> TokenStream {
    op(input, BinOp::Sum)
}

fn op(input: TokenStream, bin_op: BinOp) -> TokenStream {
    let bin_op = match bin_op {
        BinOp::Div => quote! { axcut::syntax::statements::op::BinOp::Div },
        BinOp::Prod => quote! { axcut::syntax::statements::op::BinOp::Prod },
        BinOp::Rem => quote! { axcut::syntax::statements::op::BinOp::Rem },
        BinOp::Sum => quote! { axcut::syntax::statements::op::BinOp::Sum },
        BinOp::Sub => quote! { axcut::syntax::statements::op::BinOp::Sub },
    };
    let args = parse_op_args(input);
    let fst = &args[0];
    let snd = &args[1];
    let var = &args[2];
    let next = &args[3];
    let free_vars = &args[4];

    quote! {
        axcut::syntax::statements::op::Op{
            fst: #fst,
            op: #bin_op,
            snd: #snd,
            var: #var,
            next: #next,
            free_vars_next:#free_vars,
        }
    }
    .into()
}

fn parse_op_args(input: TokenStream) -> Vec<proc_macro2::TokenStream> {
    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(input.into())
        .expect("Macro arguments could not be parsed")
        .into_iter()
        .collect::<Vec<_>>();
    let fst = expr_to_str(&parsed[0]);
    let snd = expr_to_str(&parsed[1]);
    let var = &parsed[2];
    let next = &parsed[3];
    let free_vars = match parsed.get(4) {
        None => quote! { std::option::Option::None },
        Some(vars) => {
            let vars = expr_to_array(vars)
                .into_iter()
                .map(|expr| {
                    let var = expr_to_str(&expr);
                    quote! { #var.to_string() }
                })
                .collect::<Vec<_>>();
            quote! { std::option::Option::Some(::std::collections::HashSet::from([ #(#vars),*])) }
        }
    };
    vec![
        quote! { #fst.to_string() },
        quote! { #snd.to_string() },
        quote! { #var.to_string() },
        quote! { ::std::rc::Rc::new( axcut::syntax::statements::Statement::from(#next)) },
        free_vars,
    ]
}
