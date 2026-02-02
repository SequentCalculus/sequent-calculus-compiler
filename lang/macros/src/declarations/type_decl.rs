use crate::utils::{expr_to_array, expr_to_str, parse_args};
use core_lang::syntax::declaration::{Codata, Data, Polarity};
use proc_macro::TokenStream;
use quote::quote;

pub fn data(input: TokenStream) -> TokenStream {
    type_decl(input, Data)
}

pub fn codata(input: TokenStream) -> TokenStream {
    type_decl(input, Codata)
}

fn type_decl<P>(input: TokenStream, dat: P) -> TokenStream
where
    P: Polarity,
{
    let dat = if dat.is_data() {
        quote! { core_lang::syntax::declaration::Data }
    } else {
        quote! { core_lang::syntax::declaration::Codata }
    };

    let args = parse_args(input, &["Type Name", "Xtors"], false);
    let name = expr_to_str(&args[0], 0);
    let xtors = expr_to_array(&args[1], 1);
    quote! {
        core_lang::syntax::declaration::TypeDeclaration{
            dat: #dat,
            name: #name.to_string(),
            xtors: ::std::vec::Vec::from([
                #(#xtors),*
            ])
        }
    }
    .into()
}
