use macro_utils::parse_args;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, Lit};

pub fn tvar(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Type Variable"], &[]);
    let var_expr = &args[0];

    quote! {
        core_lang::syntax::types::Ty::Var(#var_expr)
    }
    .into()
}

pub fn ty(input: TokenStream) -> TokenStream {
    // Accept optional second argument for explicit type arguments
    let args = parse_args(
        input.into(),
        ["Type Name", "Type Args"],
        &[(1, syn::parse_str("[]").unwrap())],
    );
    let ty_expr = &args[0];
    let ty_args_expr = &args[1];

    // special-case for the literal string "int"
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = ty_expr
        && s.value() == "int"
    {
        quote! { core_lang::syntax::types::Ty::I64 }.into()
    } else {
        quote! {
            core_lang::syntax::types::Ty::Decl {
                name: #ty_expr,
                type_args: core_lang::syntax::types::TypeArgs { args: ::std::vec::Vec::from(#ty_args_expr) },
            }
        }
        .into()
    }
}
