use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprArray, ExprLit, ExprPath, ExprTuple, Ident, Lit, Path, Token, parse::Parser,
    punctuated::Punctuated, spanned::Spanned,
};

pub fn expr_to_string(expr: &Expr, number_of_args: usize) -> String {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => s.value(),
        _ => panic!("Please provide string literal (argument {number_of_args})"),
    }
}

pub fn expr_to_array(expr: &Expr, number_of_args: usize) -> Vec<Expr> {
    match expr {
        Expr::Array(ExprArray { elems, .. }) => elems.into_iter().cloned().collect(),
        _ => panic!("Please provide an array expression (argument {number_of_args})"),
    }
}

pub fn expr_to_tuple(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::Tuple(ExprTuple { elems, .. }) => elems.into_iter().cloned().collect(),
        _ => panic!("Please provide a tuple expression"),
    }
}

pub fn is_none(expr: &Expr) -> bool {
    match expr {
        Expr::Path(ExprPath {
            path: Path { segments, .. },
            ..
        }) => {
            let Some(last) = segments.last() else {
                return false;
            };
            last.ident == Ident::new("None", last.span())
        }
        _ => false,
    }
}

/// Parse macro arguments from an input stream
/// Arguments in the `args` list will be parsed in the given order
/// any index in `skip_indices` will default to the given expression if there are less than
/// `args.len()` arguments provided
/// # Panics
/// This function panics if the arguments could not be parsed or ir the total number of arguments
/// is less than `args.len() - skip_indices.len()`
pub fn parse_args(input: TokenStream, args: &[&str], skip_indices: &[(usize, Expr)]) -> Vec<Expr> {
    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(input.into())
        .expect("Macro arguments could not be parsed")
        .into_iter()
        .collect::<Vec<_>>();
    let num_parsed = parsed.len();

    let mut parsed_iter = parsed.into_iter();
    let mut exprs = Vec::with_capacity(args.len());
    let to_skip = args.len() - num_parsed;
    let mut skip_indices = skip_indices.to_vec();
    skip_indices.reverse();
    skip_indices.truncate(to_skip);
    skip_indices.reverse();
    for (ind, arg_name) in args.into_iter().enumerate() {
        if let Some((_, default)) = skip_indices.iter().find(|(skip_ind, _)| *skip_ind == ind) {
            exprs.push(default.clone());
            continue;
        }

        let err_msg = format!("Please provide {arg_name}");
        exprs.push(parsed_iter.next().expect(&err_msg));
    }
    exprs
}

pub fn quote_option(
    expr: &Expr,
    some_fun: impl Fn(&Expr) -> proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if is_none(expr) {
        quote! { #expr }
    } else {
        let mapped = some_fun(expr);
        quote! { ::std::option::Option::Some(#mapped) }
    }
}
