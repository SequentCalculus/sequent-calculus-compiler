use macro_utils::{expr_to_array, expr_to_string, expr_to_tuple, parse_args};
use proc_macro::TokenStream;
use quote::quote;

pub fn substitute(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Rearrange", "Next Statement"], &[]);
    let rearrange = expr_to_array(&args[0], 0).into_iter().map(|expr| {
        let tuple_elems = expr_to_tuple(&expr);
        let binding = &tuple_elems[0];
        let var = expr_to_string(&tuple_elems[1], 1);
        quote! { (#binding,#var.to_string()) }
    });
    let next = &args[1];
    quote! {
        axcut::syntax::statements::substitute::Substitute{
            rearrange: ::std::vec::Vec::from([
                #(#rearrange),*
            ]),
            next: ::std::rc::Rc::new(axcut::syntax::statements::Statement::from(#next))
        }
    }
    .into()
}
