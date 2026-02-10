use core_lang::syntax::declaration::{Codata, Data, Polarity};
use macro_utils::{expr_to_array, expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn data(input: TokenStream) -> TokenStream {
    type_decl(input, Data)
}

pub fn codata(input: TokenStream) -> TokenStream {
    type_decl(input, Codata)
}

fn type_decl<P: Polarity>(input: TokenStream, polarity: P) -> TokenStream {
    let polarity = if polarity.is_data() {
        quote! { core_lang::syntax::declaration::Data }
    } else {
        quote! { core_lang::syntax::declaration::Codata }
    };

    let args = parse_args(input.into(), ["Type Name", "Xtors"], &[]);
    let name = expr_to_string(&args[0], 0);
    let xtors = expr_to_array(&args[1], 1);
    quote! {
        core_lang::syntax::declaration::TypeDeclaration{
            dat: #polarity,
            name: #name.to_string(),
            xtors: ::std::vec::Vec::from([
                #(#xtors),*
            ])
        }
    }
    .into()
}
