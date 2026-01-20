use crate::{
    arguments::arguments,
    utils::{expr_to_str, parse_args},
};
use proc_macro::TokenStream;
use quote::quote;

pub fn exit(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Exit Term"], true);
    let exit_term = &args[0];
    let exit_ty = &args[1];
    quote! { core_lang::syntax::statements::exit::Exit{
        arg: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exit_term)),
        ty: #exit_ty
        }
    }
    .into()
}
