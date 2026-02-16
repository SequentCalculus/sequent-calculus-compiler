use macro_utils::parse_args;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, Lit};

pub fn ty(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Type Name"], &[]);
    let ty = &args[0];
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = ty
        && s.value() == "int"
    {
        quote! {axcut::syntax::types::Ty::I64}
    } else {
        quote! {axcut::syntax::types::Ty::Decl(#ty)}
    }
    .into()
}
