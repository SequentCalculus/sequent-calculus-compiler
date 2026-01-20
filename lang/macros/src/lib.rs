use core_lang::syntax::Chirality;
use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

mod args;
mod utils;
use args::parse_args;
use utils::{expr_to_array, expr_to_str};

#[doc = include_str!("../doc/ty.md")]
#[proc_macro]
pub fn ty(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Type Name"], false);
    let ty = expr_to_str(&args[0]);
    if ty == "int" {
        quote! {core_lang::syntax::types::Ty::I64}
    } else {
        quote! {core_lang::syntax::types::Ty::Decl(#ty.to_string())}
    }
    .into()
}

fn arguments(arg: &Expr) -> proc_macro2::TokenStream {
    let args = expr_to_array(arg)
        .iter()
        .map(|arg| quote! { core_lang::syntax::terms::Term::from(#arg).into() })
        .collect::<Vec<_>>();
    quote! {
        core_lang::syntax::arguments::Arguments { entries: ::std::vec::Vec::from([
            #(#args),*
        ]) }
    }
}

fn xtor(input: TokenStream, prdcns: Chirality) -> TokenStream {
    let (chi, xtor_desc) = match prdcns {
        Chirality::Prd => (quote! { core_lang::syntax::Prd}, "Ctor Name"),
        Chirality::Cns => (quote! { core_lang::syntax::Cns}, "Dtor Name"),
    };

    let args = parse_args(input, &[xtor_desc, "Argument list"], true);

    let xtor_name = expr_to_str(&args[0]);
    let xtor_args = arguments(&args[1]);
    let ty = &args[2];
    quote! {
        core_lang::syntax::terms::xtor::Xtor{
            prdcns: #chi,
            id: #xtor_name.to_string(),
            args: #xtor_args,
            ty: #ty
        }
    }
    .into()
}

fn xvar(input: TokenStream, prdcns: Chirality) -> TokenStream {
    let prdcns = match prdcns {
        Chirality::Prd => quote! {core_lang::syntax::terms::Prd},
        Chirality::Cns => quote! {core_lang::syntax::terms::Cns},
    };
    let args = parse_args(input, &["Variable Name"], true);
    let var_name = expr_to_str(&args[0]);
    let var_ty = &args[1];
    quote! {
        core_lang::syntax::terms::xvar::XVar{
            prdcns: #prdcns,
            var: #var_name.to_string(),
            ty: #var_ty
        }
    }
    .into()
}

#[doc = include_str!("../doc/var.md")]
#[proc_macro]
pub fn var(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Prd)
}

#[doc = include_str!("../doc/covar.md")]
#[proc_macro]
pub fn covar(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Cns)
}

#[doc = include_str!("../doc/cut.md")]
#[proc_macro]
pub fn cut(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Producer", "Consumer"], true);
    let prod = &args[0];
    let cons = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::statements::Cut{
            producer: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#prod)),
            consumer: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#cons)),
            ty:#ty
        }
    }
    .into()
}

#[doc = include_str!("../doc/ctor.md")]
#[proc_macro]
pub fn ctor(input: TokenStream) -> TokenStream {
    xtor(input, Chirality::Prd)
}

#[doc = include_str!("../doc/dtor.md")]
#[proc_macro]
pub fn dtor(input: TokenStream) -> TokenStream {
    xtor(input, Chirality::Cns)
}

#[doc = include_str!("../doc/ifc.md")]
#[proc_macro]
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

#[doc = include_str!("../doc/ifcz.md")]
#[proc_macro]
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

#[doc = include_str!("../doc/call.md")]
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Called Name", "Arguments"], true);
    let name = expr_to_str(&args[0]);
    let call_args = arguments(&args[1]);
    let call_ty = &args[2];
    quote! {
        core_lang::syntax::statements::call::Call{
            name: #name.to_string(),
            args: #call_args,
            ty:#call_ty
        }
    }
    .into()
}

#[doc = include_str!("../doc/exit.md")]
#[proc_macro]
pub fn exit(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Exit Term"], true);
    let exit_term = &args[0];
    let exit_ty = &args[1];
    quote! { core_lang::syntax::statements::exit::Exit{
        arg: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exit_term)),
        ty: #exit_ty
        }
    }
    .into()
}

#[doc = include_str!("../doc/bind.md")]
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    let args = parse_args(input, &["Context Variable", "Context Chirality"], true);
    let var = expr_to_str(&args[0]);
    let chi = &args[1];
    let ty = &args[2];
    quote! {
        core_lang::syntax::context::ContextBinding{
            var: #var.to_string(),
            chi: #chi,
            ty: #ty
        }
    }
    .into()
}

#[doc = include_str!("../doc/def.md")]
#[proc_macro]
pub fn def(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &["Def Name", "Def Args", "Def Body", "Def Used Vars"],
        false,
    );
    let name = expr_to_str(&args[0]);
    let def_args = expr_to_array(&args[1]);
    let def_body = &args[2];
    let def_used = expr_to_array(&args[3])
        .iter()
        .map(|arg| quote! { #arg.to_string() })
        .collect::<Vec<_>>();
    quote! {
        core_lang::syntax::def::Def{
            name: #name.to_string(),
            context: core_lang::syntax::context::TypingContext{
                bindings: ::std::vec::Vec::from([
                    #(#def_args),*
                ]),
            },
            body: core_lang::syntax::statements::Statement::from(#def_body),
            used_vars: std::collections::HashSet::from([#(#def_used),*])
        }
    }
    .into()
}

#[doc = include_str!("../doc/prog.md")]
#[proc_macro]
pub fn prog(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &["Definitions", "Data Declarations", "Codata Declarations"],
        false,
    );
    let def_list = expr_to_array(&args[0]);
    let data_list = expr_to_array(&args[1]);
    let codata_list = expr_to_array(&args[2]);
    quote! {
        core_lang::syntax::program::Prog{
            defs: ::std::vec::Vec::from([
                      #(#def_list),*
            ]),
            data_types: ::std::vec::Vec::from([
                #(#data_list),*
            ]),
            codata_types: ::std::vec::Vec::from([
                #(#codata_list),*
            ])
        }
    }
    .into()
}

#[doc=include_str!("../doc/op.md")]
#[proc_macro]
pub fn op(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &["First Operand", "Operation", "Second Operand"],
        false,
    );
    let fst = &args[0];
    let op = &args[1];
    let snd = &args[2];
    quote! {
        core_lang::syntax::terms::op::Op{
            fst: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#fst)),
            op: #op,
            snd: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#snd)),
        }
    }
    .into()
}

#[doc = include_str!("../doc/clause.md")]
#[proc_macro]
pub fn clause(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input,
        &["Chirality", "Xtor Name", "Xtor Arguments", "Clause Body"],
        false,
    );
    let chi = &args[0];
    let xtor = expr_to_str(&args[1]);
    let xtor_args = expr_to_array(&args[2]);
    let body = &args[3];

    quote! {
        core_lang::syntax::terms::clause::Clause{
            prdcns: #chi,
            xtor: #xtor.to_string(),
            context: core_lang::syntax::context::TypingContext{
                bindings: Vec::from([
                    #(#xtor_args),*
                ])
            },
            body: ::std::rc::Rc::new(core_lang::syntax::statements::Statement::from(#body))
        }
    }
    .into()
}

fn xcase(input: TokenStream, prdcns: Chirality) -> TokenStream {
    let prdcns = match prdcns {
        Chirality::Prd => quote! {core_lang::syntax::Prd},
        Chirality::Cns => quote! {core_lang::syntax::Cns},
    };
    let args = parse_args(input, &["Case Clauses"], true);
    let clauses = expr_to_array(&args[0]);
    let ty = &args[1];
    quote! {
        core_lang::syntax::terms::xcase::XCase{
            prdcns: #prdcns,
            clauses: ::std::vec::Vec::from([
                #(#clauses),*
            ]),
            ty: #ty
        }
    }
    .into()
}

#[doc=include_str!("../doc/case.md")]
#[proc_macro]
pub fn case(input: TokenStream) -> TokenStream {
    xcase(input, Chirality::Cns)
}

#[doc=include_str!("../doc/cocase.md")]
#[proc_macro]
pub fn cocase(input: TokenStream) -> TokenStream {
    xcase(input, Chirality::Prd)
}
