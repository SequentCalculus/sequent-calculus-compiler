use crate::utils::{expr_to_str, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

pub fn unfocused_ifc(input: TokenStream) -> TokenStream {
    ifc(
        input,
        true,
        |exp| quote! {::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exp))},
        quote! {core_lang::syntax::statements::Statement},
    )
}

pub fn fs_ifc(input: TokenStream) -> TokenStream {
    ifc(
        input,
        true,
        |exp| {
            let var = expr_to_str(exp);
            quote! { #var.to_string() }
        },
        quote! {core_lang::syntax::statements::FsStatement},
    )
}

pub fn unfocused_ifcz(input: TokenStream) -> TokenStream {
    ifc(
        input,
        false,
        |exp| {
            quote! { ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exp))
            }
        },
        quote! {core_lang::syntax::statements::Statement },
    )
}

pub fn fs_ifcz(input: TokenStream) -> TokenStream {
    ifc(
        input,
        false,
        |exp| {
            let var = expr_to_str(exp);
            quote! { #var.to_string() }
        },
        quote! {core_lang::syntax::statements::FsStatement},
    )
}

fn ifc(
    input: TokenStream,
    include_snd: bool,
    prod_ty: fn(&Expr) -> proc_macro2::TokenStream,
    stmt_ty: proc_macro2::TokenStream,
) -> TokenStream {
    let arg_names = if include_snd {
        [
            "If Sort",
            "If first",
            "If Second",
            "Then Clause",
            "Else Clause",
        ]
        .as_slice()
    } else {
        ["If Sort", "If First", "Then Clause", "Else Clause"].as_slice()
    };
    let args = parse_args(input, arg_names, false);

    let mut ind = 0;
    let sort = &args[ind];
    ind += 1;
    let fst = prod_ty(&args[ind]);
    ind += 1;
    let snd = if include_snd {
        let snd = prod_ty(&args[ind]);
        ind += 1;
        quote! { ::core::option::Option::Some(#snd) }
    } else {
        quote! { ::core::option::Option::None }
    };
    let thenc = &args[ind];
    ind += 1;
    let elsec = &args[ind];
    quote! {
        core_lang::syntax::statements::ifc::IfC{
            sort: #sort,
            fst: #fst,
            snd: #snd,
            thenc: ::std::rc::Rc::new(#stmt_ty::from(#thenc)),
            elsec: ::std::rc::Rc::new(#stmt_ty::from(#elsec))
        }
    }
    .into()
}
