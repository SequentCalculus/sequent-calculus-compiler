<<<<<<< HEAD
use crate::utils::expr_to_string;
=======
use crate::utils::expr_to_str;
>>>>>>> 47eb428 (updated ifc macros)
use core_lang::syntax::statements::ifc::IfSort;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Token, parse::Parser, punctuated::Punctuated};

pub fn unfocused_ife(input: TokenStream) -> TokenStream {
    unfocused_if(input, IfSort::Equal)
}
pub fn unfocused_ifne(input: TokenStream) -> TokenStream {
    unfocused_if(input, IfSort::NotEqual)
}
pub fn unfocused_ifl(input: TokenStream) -> TokenStream {
    unfocused_if(input, IfSort::Less)
}
pub fn unfocused_ifle(input: TokenStream) -> TokenStream {
    unfocused_if(input, IfSort::LessOrEqual)
}
pub fn unfocused_ifg(input: TokenStream) -> TokenStream {
    unfocused_if(input, IfSort::Greater)
}
pub fn unfocused_ifge(input: TokenStream) -> TokenStream {
    unfocused_if(input, IfSort::GreaterOrEqual)
}

pub fn fs_ife(input: TokenStream) -> TokenStream {
    fs_if(input, IfSort::Equal)
}
pub fn fs_ifne(input: TokenStream) -> TokenStream {
    fs_if(input, IfSort::NotEqual)
}
pub fn fs_ifl(input: TokenStream) -> TokenStream {
    fs_if(input, IfSort::Less)
}
pub fn fs_ifle(input: TokenStream) -> TokenStream {
    fs_if(input, IfSort::LessOrEqual)
}
pub fn fs_ifg(input: TokenStream) -> TokenStream {
    fs_if(input, IfSort::Greater)
}
pub fn fs_ifge(input: TokenStream) -> TokenStream {
    fs_if(input, IfSort::GreaterOrEqual)
}

fn unfocused_if(input: TokenStream, sort: IfSort) -> TokenStream {
    ifc(
        input,
        sort,
<<<<<<< HEAD
        |exp, _| quote! {::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exp))},
=======
        |exp| quote! {::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exp))},
>>>>>>> 47eb428 (updated ifc macros)
        quote! {core_lang::syntax::statements::Statement},
    )
}

pub fn fs_if(input: TokenStream, sort: IfSort) -> TokenStream {
    ifc(
        input,
        sort,
<<<<<<< HEAD
        |exp, num_arg| {
            let var = expr_to_string(exp, num_arg);
=======
        |exp| {
            let var = expr_to_str(exp);
>>>>>>> 47eb428 (updated ifc macros)
            quote! { #var.to_string() }
        },
        quote! {core_lang::syntax::statements::FsStatement},
    )
}

fn ifc(
    input: TokenStream,
    sort: IfSort,
<<<<<<< HEAD
    arg_converter: fn(&Expr, usize) -> proc_macro2::TokenStream,
    statement_kind: proc_macro2::TokenStream,
=======
    prod_ty: fn(&Expr) -> proc_macro2::TokenStream,
    stmt_ty: proc_macro2::TokenStream,
>>>>>>> 47eb428 (updated ifc macros)
) -> TokenStream {
    let sort = match sort {
        IfSort::Equal => quote! {core_lang::syntax::statements::ifc::IfSort::Equal},
        IfSort::NotEqual => quote! {core_lang::syntax::statements::ifc::IfSort::NotEqual},
        IfSort::Less => quote! {core_lang::syntax::statements::ifc::IfSort::Less},
        IfSort::LessOrEqual => quote! {core_lang::syntax::statements::ifc::IfSort::LessOrEqual},
        IfSort::Greater => quote! {core_lang::syntax::statements::ifc::IfSort::Greater},
        IfSort::GreaterOrEqual => {
            quote! {core_lang::syntax::statements::ifc::IfSort::GreaterOrEqual}
        }
    };
<<<<<<< HEAD
    let args = parse_if_args(input, arg_converter);
=======
    let args = parse_if_args(input, prod_ty);
>>>>>>> 47eb428 (updated ifc macros)
    let fst = &args[0];
    let snd = &args[1];
    let thenc = &args[2];
    let elsec = &args[3];

    quote! {
        core_lang::syntax::statements::ifc::IfC{
            sort: #sort,
            fst: #fst,
            snd: #snd,
            thenc: ::std::rc::Rc::new(#statement_kind::from(#thenc)),
            elsec: ::std::rc::Rc::new(#statement_kind::from(#elsec))
        }
    }
    .into()
}

fn parse_if_args(
    input: TokenStream,
<<<<<<< HEAD
    arg_converter: fn(&Expr, usize) -> proc_macro2::TokenStream,
=======
    prod_ty: fn(&Expr) -> proc_macro2::TokenStream,
>>>>>>> 47eb428 (updated ifc macros)
) -> Vec<proc_macro2::TokenStream> {
    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(input.into())
        .expect("Macro arguments could not be parsed")
        .into_iter()
        .collect::<Vec<Expr>>();
    let mut ind = 0;
<<<<<<< HEAD
    let fst = arg_converter(&parsed[0], 0);
    ind += 1;

    let snd = if parsed.len() == 4 {
        let snd = arg_converter(&parsed[ind], ind);
=======
    let fst = prod_ty(&parsed[0]);
    ind += 1;

    let snd = if parsed.len() == 4 {
        let snd = prod_ty(&parsed[ind]);
>>>>>>> 47eb428 (updated ifc macros)
        ind += 1;
        quote! { ::core::option::Option::Some(#snd) }
    } else {
        quote! { ::core::option::Option::None }
    };

    let thenc = {
        let thenc = &parsed[ind];
        quote! {#thenc}
    };
    ind += 1;
    let elsec = {
        let elsec = &parsed[ind];
        quote! {#elsec}
    };

    vec![fst, snd, thenc, elsec]
}
