use core_lang::syntax::context::Chirality;
use macro_utils::{expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn xmu(
    input: TokenStream,
    prdcns: Chirality,
    statement_ty: proc_macro2::TokenStream,
) -> TokenStream {
    let prdcns = match prdcns {
        Chirality::Prd => quote! { core_lang::syntax::Prd },
        Chirality::Cns => quote! { core_lang::syntax::Cns },
    };
    let args = parse_args(
        input.into(),
        ["Bound Variable", "Bound Statement", "Type"],
        &[(2, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
    let var = expr_to_string(&args[0], 0);
    let stmt = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::terms::mu::Mu{
            prdcns: #prdcns,
            variable: #var.to_string(),
            statement: ::std::rc::Rc::new(#statement_ty::from(#stmt)),
            ty: #ty
        }
    }
    .into()
}

pub fn unfocused_xmu(input: TokenStream, prdcns: Chirality) -> TokenStream {
    xmu(
        input,
        prdcns,
        quote! {core_lang::syntax::statements::Statement},
    )
}

pub fn fs_xmu(input: TokenStream, prdcns: Chirality) -> TokenStream {
    xmu(
        input,
        prdcns,
        quote! {core_lang::syntax::statements::FsStatement},
    )
}
