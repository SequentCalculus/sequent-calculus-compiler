use crate::utils::{expr_to_array, expr_to_string, expr_to_tuple, parse_args};
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
        input,
        &["Def Name", "Def Args", "Def Body", "Def Used Vars"],
        false,
    );
    let name = expr_to_string(&args[0], 0);
    let def_args = expr_to_array(&args[1], 1);
    let def_body = &args[2];
    let def_used = expr_to_array(&args[3], 3)
        .iter()
        .enumerate()
        .map(|(ind, arg)| {
            let var_arr = expr_to_tuple(arg);
            let var_name = expr_to_string(&var_arr[0], ind);
            let var_id = &var_arr[1];
            quote! { core_lang::syntax::names::Var {
                name: #var_name.to_string(),
                id: #var_id
            }}
        })
        .collect::<Vec<_>>();
    quote! {
        core_lang::syntax::def::Def{
            name: #name.to_string(),
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
