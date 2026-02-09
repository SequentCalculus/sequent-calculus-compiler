use macro_utils::{expr_to_array, expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn clause(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Clause Xtor", "Clause Context", "Clause Body"],
        &[],
    );
    let xtor = expr_to_string(&args[0], 0);
    let ctx = expr_to_array(&args[1], 1);
    let body = &args[2];
    quote! {
        axcut::syntax::statements::clause::Clause{
            xtor: #xtor.to_string(),
            context: axcut::syntax::context::TypingContext{
                bindings: ::std::vec::Vec::from([
                    #(#ctx),*
                ])
            },
            body: ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#body))
        }
    }
    .into()
}
