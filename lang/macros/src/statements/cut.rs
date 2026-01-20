use crate::utils::parse_args;
use proc_macro::TokenStream;
use quote::quote;

pub fn cut(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Producer", "Consumer"], true);
    let prod = &args[0];
    let cons = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::statements::Cut{
            producer: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#prod)),
            consumer: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#cons)),
            ty:#ty
        }
    }
    .into()
}
