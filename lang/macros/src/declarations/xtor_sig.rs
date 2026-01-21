use crate::utils::{expr_to_array, expr_to_str, parse_args};
use core_lang::syntax::declaration::{Codata, Data, Polarity};
use proc_macro::TokenStream;
use quote::quote;

pub fn ctor_sig(input: TokenStream) -> TokenStream {
    xtor_sig(input, Data)
}

pub fn dtor_sig(input: TokenStream) -> TokenStream {
    xtor_sig(input, Codata)
}

fn xtor_sig<P>(input: TokenStream, dat: P) -> TokenStream
where
    P: Polarity,
{
    let dat = if dat.is_data() {
        quote! {core_lang::syntax::declaration::Data}
    } else {
        quote! {core_lang::syntax::declaration::Codata}
    };
    let args = parse_args(input, &["Xtor Name", "Xtor Args"], false);
    let name = expr_to_str(&args[0]);
    let xtor_args = expr_to_array(&args[1]);
    quote! {
        core_lang::syntax::declaration::XtorSig{
            xtor: #dat,
            name: #name.to_string(),
            args: core_lang::syntax::context::TypingContext {
                bindings: ::std::vec::Vec::from([
                    #(#xtor_args),*
                ])
            }
        }
    }
    .into()
}
