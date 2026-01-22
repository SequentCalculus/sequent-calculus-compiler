use proc_macro2::TokenStream;
use syn::{Expr, ExprArray, ExprLit, ExprPath, ExprTuple, Ident, Lit, Path, spanned::Spanned};
use syn::{Token, parse::Parser, parse_str, punctuated::Punctuated};

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
/// if `optional_ty` is set, after parsing the arg_names a [`core_lang::syntax::types::Ty`] will be added as well
/// if none is provided it will default to [`core_lang::syntax::types::Ty::I64`]
pub fn parse_args(input: TokenStream, arg_names: &[&str], optional_ty: bool) -> Vec<Expr> {
    let mut parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(input.into())
        .expect("Macro arguments could not be parsed")
        .into_iter();
    let mut args = Vec::with_capacity(arg_names.len() + 1);
    for arg_name in arg_names {
        let err_msg = format!("Please provide {arg_name}");
        args.push(parsed.next().expect(&err_msg));
    }
    if optional_ty {
        if let Some(ty) = parsed.next() {
            args.push(ty)
        } else {
            args.push(parse_str("core_lang::syntax::types::Ty::I64").expect("Could not parse Type"))
        }
    }
    args
}
