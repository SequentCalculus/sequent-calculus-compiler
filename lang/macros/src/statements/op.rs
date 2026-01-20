use crate::utils::{expr_to_str, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

pub fn unfocused_op(input: TokenStream) -> TokenStream {
    op(input, |exp| {
        quote! {
            ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exp))
        }
    })
}

pub fn fs_op(input: TokenStream) -> TokenStream {
    op(input, |exp| {
        let var = expr_to_str(exp);
        quote! {
                #var.to_string()
        }
    })
}

fn op(input: TokenStream, prod_fun: fn(&Expr) -> proc_macro2::TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &["First Operand", "Operation", "Second Operand"],
        false,
    );
    let fst = prod_fun(&args[0]);
    let op = &args[1];
    let snd = prod_fun(&args[2]);
    quote! {
        core_lang::syntax::terms::op::Op{
            fst: #fst,
            op: #op,
            snd: #snd,
        }
    }
    .into()
}
