use core_lang::syntax::terms::op::BinOp;
use macro_utils::{expr_to_string, expr_to_tuple, parse_args};
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
    op(input, bin_op, |exp, _| {
        quote! {
            ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exp))
        }
    })
}

fn fs_op(input: TokenStream, bin_op: BinOp) -> TokenStream {
    op(input, bin_op, |exp, num_arg| {
        let var = expr_to_tuple(exp);
        let var_name = expr_to_string(&var[0], num_arg);
        let var_id = &var[1];
        quote! {
            core_lang::syntax::names::Var {
                name:#var_name.to_string(),
                id:#var_id
            }
        }
    })
}

fn op(
    input: TokenStream,
    op: BinOp,
    arg_converter: fn(&Expr, usize) -> proc_macro2::TokenStream,
) -> TokenStream {
    let args = parse_args(input.into(), ["First Operand", "Second Operand"], &[]);
    let op = match op {
        BinOp::Div => quote! {core_lang::syntax::terms::op::BinOp::Div},
        BinOp::Prod => quote! {core_lang::syntax::terms::op::BinOp::Prod},
        BinOp::Rem => quote! {core_lang::syntax::terms::op::BinOp::Rem},
        BinOp::Sum => quote! {core_lang::syntax::terms::op::BinOp::Sum},
        BinOp::Sub => quote! {core_lang::syntax::terms::op::BinOp::Sub},
    };
    let fst = arg_converter(&args[0], 0);
    let snd = arg_converter(&args[1], 1);
    quote! {
        core_lang::syntax::terms::op::Op{
            fst: #fst,
            op: #op,
            snd: #snd,
        }
    }
    .into()
}
