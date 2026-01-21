use crate::utils::{expr_to_str, parse_args};
use core_lang::syntax::terms::op::BinOp;
use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

pub fn unfocused_div(input: TokenStream) -> TokenStream {
    unfocused_op(input, BinOp::Div)
}
pub fn unfocused_prod(input: TokenStream) -> TokenStream {
    unfocused_op(input, BinOp::Prod)
}
pub fn unfocused_rem(input: TokenStream) -> TokenStream {
    unfocused_op(input, BinOp::Rem)
}
pub fn unfocused_sum(input: TokenStream) -> TokenStream {
    unfocused_op(input, BinOp::Sum)
}
pub fn unfocused_sub(input: TokenStream) -> TokenStream {
    unfocused_op(input, BinOp::Sub)
}

pub fn fs_div(input: TokenStream) -> TokenStream {
    fs_op(input, BinOp::Div)
}
pub fn fs_prod(input: TokenStream) -> TokenStream {
    fs_op(input, BinOp::Prod)
}
pub fn fs_rem(input: TokenStream) -> TokenStream {
    fs_op(input, BinOp::Rem)
}
pub fn fs_sum(input: TokenStream) -> TokenStream {
    fs_op(input, BinOp::Sum)
}
pub fn fs_sub(input: TokenStream) -> TokenStream {
    fs_op(input, BinOp::Sub)
}

fn unfocused_op(input: TokenStream, bin_op: BinOp) -> TokenStream {
    op(input, bin_op, |exp| {
        quote! {
            ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exp))
        }
    })
}

fn fs_op(input: TokenStream, bin_op: BinOp) -> TokenStream {
    op(input, bin_op, |exp| {
        let var = expr_to_str(exp);
        quote! {
                #var.to_string()
        }
    })
}

fn op(
    input: TokenStream,
    op: BinOp,
    prod_fun: fn(&Expr) -> proc_macro2::TokenStream,
) -> TokenStream {
    let args = parse_args(input, &["First Operand", "Second Operand"], false);
    let op = match op {
        BinOp::Div => quote! {core_lang::syntax::terms::op::BinOp::Div},
        BinOp::Prod => quote! {core_lang::syntax::terms::op::BinOp::Prod},
        BinOp::Rem => quote! {core_lang::syntax::terms::op::BinOp::Rem},
        BinOp::Sum => quote! {core_lang::syntax::terms::op::BinOp::Sum},
        BinOp::Sub => quote! {core_lang::syntax::terms::op::BinOp::Sub},
    };
    let fst = prod_fun(&args[0]);
    let snd = prod_fun(&args[1]);
    quote! {
        core_lang::syntax::terms::op::Op{
            fst: #fst,
            op: #op,
            snd: #snd,
        }
    }
    .into()
}
