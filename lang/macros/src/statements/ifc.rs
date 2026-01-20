use crate::utils::parse_args;
use proc_macro::TokenStream;
use quote::quote;

pub fn ifc(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &[
            "If Sort",
            "If first",
            "If Second",
            "Then Clause",
            "Else Clause",
        ],
        false,
    );
    let sort = &args[0];
    let fst = &args[1];
    let snd = &args[2];
    let thenc = &args[3];
    let elsec = &args[4];
    quote! {
        core_lang::syntax::statements::ifc::IfC{
            sort: #sort,
            fst: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#fst)),
            snd: ::core::option::Option::Some(::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#snd))),
            thenc: ::std::rc::Rc::new(core_lang::syntax::statements::Statement::from(#thenc)),
            elsec: ::std::rc::Rc::new(core_lang::syntax::statements::Statement::from(#elsec))
        }
    }
    .into()
}

pub fn ifcz(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &["If Sort", "If First", "Then Clause", "Else Clause"],
        false,
    );
    let sort = &args[0];
    let fst = &args[1];
    let thenc = &args[2];
    let elsec = &args[3];
    quote! {
        core_lang::syntax::statements::ifc::IfC{
            sort: #sort,
            fst: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#fst)),
            snd: ::core::option::Option::None,
            thenc: ::std::rc::Rc::new(core_lang::syntax::statements::Statement::from(#thenc)),
            elsec: ::std::rc::Rc::new(core_lang::syntax::statements::Statement::from(#elsec))
        }
    }
    .into()
}
