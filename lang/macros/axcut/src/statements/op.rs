use axcut::syntax::statements::op::BinOp;
use macro_utils::{expr_to_array, expr_to_string};
use macro_utils::{parse_args, quote_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn div(input: TokenStream) -> TokenStream {
    op(input, BinOp::Div)
}
pub fn prod(input: TokenStream) -> TokenStream {
    op(input, BinOp::Prod)
}
pub fn rem(input: TokenStream) -> TokenStream {
    op(input, BinOp::Rem)
}
pub fn sub(input: TokenStream) -> TokenStream {
    op(input, BinOp::Sub)
}
pub fn sum(input: TokenStream) -> TokenStream {
    op(input, BinOp::Sum)
}

fn op(input: TokenStream, bin_op: BinOp) -> TokenStream {
    let bin_op = match bin_op {
        BinOp::Div => quote! { axcut::syntax::statements::op::BinOp::Div },
        BinOp::Prod => quote! { axcut::syntax::statements::op::BinOp::Prod },
        BinOp::Rem => quote! { axcut::syntax::statements::op::BinOp::Rem },
        BinOp::Sum => quote! { axcut::syntax::statements::op::BinOp::Sum },
        BinOp::Sub => quote! { axcut::syntax::statements::op::BinOp::Sub },
    };
    let args = parse_args(
        input.into(),
        ["Fst", "Snd", "Var", "Next", "Free Vars Next"],
        &[(4, parse_str("::std::option::Option::None").unwrap())],
    );
    let fst = expr_to_string(&args[0], 0);
    let snd = expr_to_string(&args[1], 1);
    let var = expr_to_string(&args[2], 2);
    let next = &args[3];
    let free_vars = quote_option(&args[4], |free| {
        let free_arr = expr_to_array(free, 4)
            .into_iter()
            .map(|expr| quote! { #expr.to_string() })
            .collect::<Vec<_>>();
        quote! {
            ::std::collections::HashSet::from([
                #(#free_arr),*
            ])
        }
    });

    quote! {
        axcut::syntax::statements::op::Op{
            fst: #fst.to_string(),
            op: #bin_op,
            snd: #snd.to_string(),
            var: #var.to_string(),
            next: ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#next)),
            free_vars_next:#free_vars,
        }
    }
    .into()
}
