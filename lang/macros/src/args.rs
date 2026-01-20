use proc_macro::TokenStream;
use syn::{Expr, Token, parse::Parser, parse_str, punctuated::Punctuated};

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
            args.push(parse_str("Ty::I64").expect("Could not parse Type"))
        }
    }
    args
}
