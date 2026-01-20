use crate::{
    arguments::arguments,
    utils::{expr_to_str, parse_args},
};
use proc_macro::TokenStream;
use quote::quote;

pub fn op(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &["First Operand", "Operation", "Second Operand"],
        false,
    );
    let fst = &args[0];
    let op = &args[1];
    let snd = &args[2];
    quote! {
        core_lang::syntax::terms::op::Op{
            fst: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#fst)),
            op: #op,
            snd: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#snd)),
        }
    }
    .into()
}
