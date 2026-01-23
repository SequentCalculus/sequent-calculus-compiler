use macro_utils::{expr_to_array, expr_to_str, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn unfocused_def(input: TokenStream) -> TokenStream {
    def(input, quote! {core_lang::syntax::statements::Statement})
}

pub fn fs_def(input: TokenStream) -> TokenStream {
    def(input, quote! {core_lang::syntax::statements::FsStatement})
}

fn def(input: TokenStream, stmt_ty: proc_macro2::TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Def Name", "Def Args", "Def Body", "Def Used Vars"],
        &[],
    );
    let name = expr_to_str(&args[0], 0);
    let def_args = expr_to_array(&args[1], 1);
    let def_body = &args[2];
    let def_used = expr_to_array(&args[3], 3)
        .iter()
        .map(|arg| quote! { #arg.to_string() })
        .collect::<Vec<_>>();
    quote! {
        core_lang::syntax::def::Def{
            name: #name.to_string(),
            context: core_lang::syntax::context::TypingContext{
                bindings: ::std::vec::Vec::from([
                    #(#def_args),*
                ]),
            },
            body: #stmt_ty::from(#def_body),
            used_vars: std::collections::HashSet::from([#(#def_used),*])
        }
    }
    .into()
}
