use crate::utils::expr_to_array;
use quote::quote;
use syn::Expr;

pub fn arguments(arg: &Expr) -> proc_macro2::TokenStream {
    let args = expr_to_array(arg)
        .iter()
        .map(|arg| quote! { core_lang::syntax::terms::Term::from(#arg).into() })
        .collect::<Vec<_>>();
    quote! {
        core_lang::syntax::arguments::Arguments { entries: ::std::vec::Vec::from([
            #(#args),*
        ]) }
    }
}
