use core_lang::syntax::statements::ifc::IfSort;
use macro_utils::{expr_to_string, parse_args, quote_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, parse_str};

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
        |exp, _| quote! {::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exp))},
        quote! {core_lang::syntax::statements::Statement},
    )
}

pub fn fs_if(input: TokenStream, sort: IfSort) -> TokenStream {
    ifc(
        input,
        sort,
        |exp, num_arg| {
            let var = expr_to_string(exp, num_arg);
            quote! { #var.to_string() }
        },
        quote! {core_lang::syntax::statements::FsStatement},
    )
}

fn ifc(
    input: TokenStream,
    sort: IfSort,
    arg_converter: fn(&Expr, usize) -> proc_macro2::TokenStream,
    statement_kind: proc_macro2::TokenStream,
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
    let args = parse_args(
        input.into(),
        [
            "First Argument",
            "Second Argument",
            "Then Statement",
            "Else Statement",
        ],
        &[(1, parse_str("::std::option::Option::None").unwrap())],
    );
    let fst = arg_converter(&args[0], 0);
    let snd = quote_option(&args[1], |expr| {
        let exp = arg_converter(expr, 1);
        quote!(#exp)
    });
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
