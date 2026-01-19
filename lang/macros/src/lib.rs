use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Token, parse_macro_input, punctuated::Punctuated};

/// Create a [`core_lang::syntax::statements::Cut`] with given arguments
/// ```
/// use macros::cut;
/// use core_lang::syntax::{statements::Cut, terms::xvar::XVar,types::Ty};
/// let cut1 = cut!(XVar::var("x",Ty::I64).into(),XVar::covar("a",Ty::I64).into(),Ty::I64);
/// let cut2 = Cut::new(XVar::var("x",Ty::I64),XVar::covar("a",Ty::I64),Ty::I64);
/// assert_eq!(cut1,cut2)
/// ```
#[proc_macro]
pub fn cut(input: TokenStream) -> TokenStream {
    let mut args = parse_macro_input!(input with Punctuated::<Expr,Token![,]>::parse_terminated);
    let ty = args.pop().expect("Please provide type");
    let cons = args.pop().expect("Please provied consumer");
    let prod = args.pop().expect("Please provide producer");
    quote! {
        core_lang::syntax::statements::Cut{
            producer:#prod,
            consumer:#cons,
            ty:#ty
        }
    }
    .into()
}
