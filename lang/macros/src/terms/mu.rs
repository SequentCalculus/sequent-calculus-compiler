use crate::utils::{expr_to_str, parse_args};
use core_lang::syntax::context::Chirality;
use proc_macro::TokenStream;
use quote::quote;

pub fn xmu(input: TokenStream, prdcns: Chirality) -> TokenStream {
    let prdcns = match prdcns {
        Chirality::Prd => quote! { core_lang::syntax::Prd },
        Chirality::Cns => quote! { core_lang::syntax::Cns },
    };
    let args = parse_args(input, &["Bound Variable", "Bound Statement"], true);
    let var = expr_to_str(&args[0]);
    let stmt = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::terms::mu::Mu{
            prdcns: #prdcns,
            variable: #var.to_string(),
            statement: ::std::rc::Rc::new(core_lang::syntax::statements::Statement::from(#stmt)),
            ty: #ty
        }
    }
    .into()
}
