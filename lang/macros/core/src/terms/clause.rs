use crate::utils::{expr_to_array, expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn unfocused_clause(input: TokenStream) -> TokenStream {
    clause(input, quote! {core_lang::syntax::statements::Statement })
}

pub fn fs_clause(input: TokenStream) -> TokenStream {
    clause(input, quote! {core_lang::syntax::statements::FsStatement })
}

fn clause(input: TokenStream, statement_kind: proc_macro2::TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &["Chirality", "Xtor Name", "Xtor Arguments", "Clause Body"],
        &[],
    );
    let prdcns = &args[0];
    let xtor = expr_to_string(&args[1], 1);
    let xtor_args = expr_to_array(&args[2], 2);
    let body = &args[3];

    quote! {
        core_lang::syntax::terms::clause::Clause{
            prdcns: #prdcns,
            xtor: #xtor.to_string(),
            context: core_lang::syntax::context::TypingContext{
                bindings: Vec::from([
                    #(#xtor_args),*
                ])
            },
            body: ::std::rc::Rc::new(#statement_kind::from(#body))
        }
    }
    .into()
}
