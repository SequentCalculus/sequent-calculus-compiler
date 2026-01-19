use proc_macro::TokenStream;
use syn::{Expr, ExprArray, ExprLit, Lit, Token, parse::Parser, punctuated::Punctuated};

pub fn parse_args<const N: usize>(input: TokenStream, arg_names: [&str; N]) -> [Expr; N] {
    let mut parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(input.into())
        .expect("Macro arguments could not be parsed")
        .into_iter();
    arg_names.map(|arg_name| {
        let err_msg = format!("Please provide {arg_name}");
        parsed.next().expect(&err_msg)
    })
}

pub fn expr_to_str(expr: &Expr) -> String {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => s.value(),
        _ => panic!("Please provide string literal"),
    }
}

pub fn expr_to_array(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::Array(ExprArray { elems, .. }) => elems.into_iter().cloned().collect(),
        _ => panic!("Please provide an array expression"),
    }
}
