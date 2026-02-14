use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn unfocused_def(input: TokenStream) -> TokenStream {
    def(input, quote! {core_lang::syntax::statements::Statement})
}

pub fn fs_def(input: TokenStream) -> TokenStream {
    def(input, quote! {core_lang::syntax::statements::FsStatement})
}

fn def(input: TokenStream, statement_kind: proc_macro2::TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Def Name", "Def Args", "Def Body", "Def Used Vars"],
        &[],
    );
    let name = &args[0];
    let def_args = expr_to_array(&args[1], 1);
    let def_body = &args[2];
    let def_used = expr_to_array(&args[3], 3)
        .iter()
        .map(|arg| quote! { #arg })
        .collect::<Vec<_>>();
    quote! {
        core_lang::syntax::def::Def{
            name: #name,
            context: core_lang::syntax::context::TypingContext{
                bindings: ::std::vec::Vec::from([
                    #(#def_args),*
                ]),
            },
            body: #statement_kind::from(#def_body),
            used_vars: std::collections::HashSet::from([#(#def_used),*])
        }
    }
    .into()
}
