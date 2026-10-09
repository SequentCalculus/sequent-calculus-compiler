use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn unfocused_def(input: TokenStream) -> TokenStream {
    def(input, quote! {core_lang::syntax::statements::Statement})
}

pub fn fs_def(input: TokenStream) -> TokenStream {
    def(input, quote! {core_lang::syntax::statements::FsStatement})
}

fn def(input: TokenStream, statement_kind: proc_macro2::TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Def Name", "Type Parameters", "Def Args", "Def Body"],
        &[(1, parse_str("[]").unwrap())],
    );
    let name = &args[0];
    let type_params = expr_to_array(&args[1], 1);
    let def_args = expr_to_array(&args[2], 1);
    let def_body = &args[3];
    quote! {
        core_lang::syntax::def::Def{
            name: #name,
            type_params: ::std::vec::Vec::from([
                #(#type_params),*
            ]),
            context: core_lang::syntax::context::TypingContext{
                bindings: ::std::vec::Vec::from([
                    #(#def_args),*
                ])},
            body: #statement_kind::from(#def_body),
        }
    }
    .into()
}
