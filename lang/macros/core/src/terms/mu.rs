use core_lang::syntax::context::Chirality;
use macro_utils::{expr_to_string, expr_to_tuple, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn xmu(
    input: TokenStream,
    chi: Chirality,
    statement_kind: proc_macro2::TokenStream,
) -> TokenStream {
    let prdcns = match chi {
        Chirality::Prd => quote! { core_lang::syntax::Prd },
        Chirality::Cns => quote! { core_lang::syntax::Cns },
    };
    let args = parse_args(
        input.into(),
        ["Bound Variable", "Bound Statement", "Type"],
        &[(2, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
    let var = expr_to_tuple(&args[0]);
    let var_name = expr_to_string(&var[0], 0);
    let var_id = &var[1];
    let stmt = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::terms::mu::Mu{
            prdcns: #prdcns,
            variable: core_lang::syntax::names::Var {
                 name: #var_name.to_string(),
                 id: #var_id
            },
            statement: ::std::rc::Rc::new(#statement_kind::from(#stmt)),
            ty: #ty
        }
    }
    .into()
}

pub fn unfocused_xmu(input: TokenStream, chi: Chirality) -> TokenStream {
    xmu(
        input,
        chi,
        quote! {core_lang::syntax::statements::Statement},
    )
}

pub fn fs_xmu(input: TokenStream, chi: Chirality) -> TokenStream {
    xmu(
        input,
        chi,
        quote! {core_lang::syntax::statements::FsStatement},
    )
}
