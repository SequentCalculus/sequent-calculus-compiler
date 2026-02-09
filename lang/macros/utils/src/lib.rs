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

/// Parse macro arguments from an input stream. Arguments in the `args` list will be parsed in the
/// given order. If less than `args.len()` arguments provided, then number-of-missing-arguments-many
/// arguments will be filled with default arguments from `indices_to_skip` at the indices and in the
/// order given there. All other entries in `indices_to_skip` will be discarded.
///
/// # Panics
///
/// This function panics if the arguments could not be parsed or if the total number of arguments
/// is less than `args.len() - indices_to_skip.len()`.
pub fn parse_args<const N: usize>(
    input: TokenStream,
    args: [&str; N],
    indices_to_skip: &[(usize, Expr)],
) -> [Expr; N] {
    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(input)
        .expect("Macro arguments could not be parsed")
        .into_iter()
        .collect::<Vec<_>>();
    let number_of_parsed_args = parsed.len();

    let to_skip = args.len() - number_of_parsed_args;
    let mut indices_to_skip = indices_to_skip.to_vec();
    indices_to_skip.reverse();
    indices_to_skip.truncate(to_skip);
    indices_to_skip.reverse();

    let mut parsed_iter = parsed.into_iter();
    std::array::from_fn(|index| {
        let arg_name = args[index];
        if let Some((_, default)) = indices_to_skip
            .iter()
            .find(|(skip_index, _)| *skip_index == index)
        {
            default.clone()
        } else {
            let err_msg = format!("Please provide {arg_name}");
            parsed_iter.next().expect(&err_msg)
        }
    })
}

pub fn quote_option(
    expr: &Expr,
    converter: impl Fn(&Expr) -> proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if is_none(expr) {
        quote! { #expr }
    } else {
        let mapped = converter(expr);
        quote! { ::std::option::Option::Some(#mapped) }
    }
}
