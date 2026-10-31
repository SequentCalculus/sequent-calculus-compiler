use core_lang::syntax::declaration::{Codata, Data, Polarity};
use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

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

    let args = parse_args(
        input.into(),
        ["Type Name", "Xtors", "Type Parameters"],
        &[(2, parse_str("[]").unwrap())],
    );
    let identifier = &args[0];
    let xtors = expr_to_array(&args[1], 1);
    let type_params = expr_to_array(&args[2], 2);
    quote! {
        core_lang::syntax::declaration::TypeDeclaration{
            dat: #polarity,
            name: #identifier,
            xtors: ::std::vec::Vec::from([
                #(#xtors),*
            ]),
            type_params: ::std::vec::Vec::from([
                #(#type_params),*
            ])
        }
    }
    .into()
}
