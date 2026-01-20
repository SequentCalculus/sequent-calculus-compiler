use crate::utils::{expr_to_array, expr_to_str, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn clause(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &["Chirality", "Xtor Name", "Xtor Arguments", "Clause Body"],
        false,
    );
    let chi = &args[0];
    let xtor = expr_to_str(&args[1]);
    let xtor_args = expr_to_array(&args[2]);
    let body = &args[3];

    quote! {
        core_lang::syntax::terms::clause::Clause{
            prdcns: #chi,
            xtor: #xtor.to_string(),
            context: core_lang::syntax::context::TypingContext{
                bindings: Vec::from([
                    #(#xtor_args),*
                ])
            },
            body: ::std::rc::Rc::new(core_lang::syntax::statements::Statement::from(#body))
        }
    }
    .into()
}
