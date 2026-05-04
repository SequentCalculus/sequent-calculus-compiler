use core_lang::syntax::declaration::{Codata, Data, Polarity};
use macro_utils::{expr_to_array, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn ctor_sig(input: TokenStream) -> TokenStream {
    xtor_sig(input, Data)
}

pub fn dtor_sig(input: TokenStream) -> TokenStream {
    xtor_sig(input, Codata)
}

fn xtor_sig<P: Polarity>(input: TokenStream, polarity: P) -> TokenStream {
    let polarity = if polarity.is_data() {
        quote! {core_lang::syntax::declaration::Data}
    } else {
        quote! {core_lang::syntax::declaration::Codata}
    };
    let args = parse_args(input.into(), ["Xtor Name", "Xtor Args"], &[]);
    let name = &args[0];
    let xtor_args = expr_to_array(&args[1], 1);
    quote! {
        core_lang::syntax::declaration::XtorSig{
            xtor: #polarity,
            name: #name,
            args: core_lang::syntax::context::TypingContext {
                bindings: ::std::vec::Vec::from([
                    #(#xtor_args),*
                ])
            }
        }
    }
    .into()
}
