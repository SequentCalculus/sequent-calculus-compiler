use core_lang::syntax::Chirality;
use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

mod args;
mod utils;
use args::parse_args;
use utils::{expr_to_array, expr_to_str};

/// Create a [`core_lang::syntax::types::Type`] from a string literal
/// `int` will create [`core_lang::syntax::types::Type::I64`]
/// anything else will create [`core_lang::syntax::types::Type::Decl`]
/// ```
/// use macros::ty;
/// use core_lang::syntax::types::Ty;
/// let int1 = ty!("int");
/// let int2 = Ty::I64;
/// assert_eq!(int1,int2);
/// let list1 = ty!("ListInt");
/// let list2 = Ty::Decl("ListInt".to_string());
/// assert_eq!(list1,list2)
/// ```
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

/// Create a [`core_lang::syntax::terms::xvar::XVar`] with chrality
/// [`core_lang::syntax::terms::Prd`]
/// If no type is provided, the variable will default to [`core_lang::syntax::types::Ty::I64`]
/// ```
/// use macros::{ty,var};
/// use core_lang::syntax::{types::Ty, terms::xvar::XVar};
/// let var1 = var!("x");
/// let var2 = var!("x",Ty::I64);
/// let var3 = XVar::var("x",Ty::I64);
/// assert_eq!(var1,var2);
/// assert_eq!(var2,var3);
/// ```
#[proc_macro]
pub fn var(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Prd)
}

/// Create a [`core_lang::syntax::terms::xvar::XVar`] with chirality
/// [`core_lang::syntax::terms::Cns`]
/// If no type is provided the covariable will default to [`core_lang::syntax::types::ty::I64`]
/// ```
/// use macros::covar;
/// use core_lang::syntax::{terms::xvar::XVar,types::Ty};
/// let covar1 = covar!("a");
/// let covar2 = covar!("a",Ty::I64);
/// let covar3 = XVar::covar("a",Ty::I64);
/// assert_eq!(covar1,covar2);
/// assert_eq!(covar2,covar3);
/// ```
#[proc_macro]
pub fn covar(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Cns)
}

/// Create a [`core_lang::syntax::statements::Cut`] with given arguments
/// if no type is provided, the cut type will default to [`core_lang::syntax::types::Ty::I64`]
/// ```
/// use macros::cut;
/// use core_lang::syntax::{ statements::Cut, terms::xvar::XVar,types::Ty};
/// let cut1 = cut!(XVar::var("x",Ty::I64),XVar::covar("a",Ty::I64));
/// let cut2 = cut!(XVar::var("x",Ty::I64),XVar::covar("a",Ty::I64),Ty::I64);
/// let cut3 = Cut::new(XVar::var("x",Ty::I64),XVar::covar("a",Ty::I64),Ty::I64);
/// assert_eq!(cut1,cut2);
/// assert_eq!(cut2,cut3)
/// ```
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

/// Create a [`core_lang::syntax::terms::Xtor`] with chirality [`core_lang::syntax::terms::Prd`]
/// (i.e. a constructor)
/// ```
/// use macros::ctor;
/// use core_lang::syntax::{arguments::Arguments, types::Ty, terms::{XVar, Xtor}};
/// let ctor1 = ctor!("Cons",
///     [XVar::var("x",Ty::I64),ctor!("Nil",[],Ty::Decl("ListInt".to_string()))],
///     Ty::Decl("ListInt".to_string()));
///
/// let mut arguments = Arguments::default();
/// arguments.add_prod(XVar::var("x",Ty::I64));
/// arguments.add_prod(Xtor::ctor("Nil", Arguments::default(), Ty::Decl("ListInt".to_string()),));
/// let ctor2 = Xtor::ctor("Cons",arguments,Ty::Decl("ListInt".to_string()));
/// assert_eq!(ctor1,ctor2)
/// ```
#[proc_macro]
pub fn ctor(input: TokenStream) -> TokenStream {
    xtor(input, Chirality::Prd)
}

/// Create a [`core_lang::syntax::terms::Xtor`] with chirality [`core_lang::syntax::terms::Cns`]
/// (i.e. a destructor)
/// ```
/// use macros::dtor;
/// use core_lang::syntax::{types::Ty, arguments::Arguments, terms::{XVar, Xtor}};
/// let dtor1 = dtor!("apply",
///     [XVar::var("x",Ty::I64)],
///     Ty::Decl("FunI64I64".to_string()));
///
/// let mut arguments = Arguments::default();
/// arguments.add_prod(XVar::var("x",Ty::I64));
/// let dtor2 = Xtor::dtor("apply",arguments,Ty::Decl("FunI64I64".to_string()));
/// assert_eq!(dtor1,dtor2)
/// ```
#[proc_macro]
pub fn dtor(input: TokenStream) -> TokenStream {
    xtor(input, Chirality::Cns)
}

/// Create a [`core_lang::syntax::terms::ifc::IfC`]
/// ```
/// use macros::ifc;
/// use core_lang::syntax::{types::Ty, statements::{exit::Exit,ifc::{IfSort,IfC}},terms::{Term, xvar::XVar,}};
/// use std::rc::Rc;
///
/// let if1 = ifc!(
///     IfSort::Equal,
///     XVar::var("x",Ty::I64),
///     XVar::var("y",Ty::I64),
///     Exit::exit(XVar::var("z",Ty::I64),Ty::I64),
///     Exit::exit(XVar::var("w",Ty::I64),Ty::I64),
/// );
/// let if2 = IfC{
///     sort:IfSort::Equal,
///     fst:Rc::new(Term::from(XVar::var("x",Ty::I64))),
///     snd:Some(Rc::new(Term::from(XVar::var("y",Ty::I64)))),
///     thenc:Rc::new(Exit::exit(XVar::var("z",Ty::I64),Ty::I64).into()),
///     elsec:Rc::new(Exit::exit(XVar::var("w",Ty::I64),Ty::I64).into())
///     };
/// assert_eq!(if1,if2)
/// ```
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

/// Create a [`core_lang::syntax::terms::ifc::IfC`] with comparison to zero
/// ```
/// use macros::ifcz;
/// use core_lang::syntax::{types::Ty, statements::{exit::Exit,ifc::{IfSort,IfC}},terms::{Term, xvar::XVar,}};
/// use std::rc::Rc;
///
/// let if1 = ifcz!(
///     IfSort::Equal,
///     XVar::var("x",Ty::I64),
///     Exit::exit(XVar::var("z",Ty::I64),Ty::I64),
///     Exit::exit(XVar::var("w",Ty::I64),Ty::I64),
/// );
/// let if2 = IfC{
///     sort:IfSort::Equal,
///     fst:Rc::new(Term::from(XVar::var("x",Ty::I64))),
///     snd:None,
///     thenc:Rc::new(Exit::exit(XVar::var("z",Ty::I64),Ty::I64).into()),
///     elsec:Rc::new(Exit::exit(XVar::var("w",Ty::I64),Ty::I64).into())
///     };
/// assert_eq!(if1,if2)
/// ```
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

/// Create a [`core_lang::syntax::statements::Call`]
/// if no return type is provided it will default to [`core_lang::syntax::types::Ty::I64`]
/// ```
/// use macros::call;
/// use core_lang::syntax::{arguments::{Argument,Arguments}, statements::Call, types::Ty, terms::{Term, xvar::XVar}};
/// let call1 = call!("print",[XVar::var("x",Ty::I64)],Ty::I64);
/// let call2 = call!("print",[XVar::var("x",Ty::I64)]);
/// let call3 = Call{
///     name:"print".to_string(),
///     args:Arguments{entries:Vec::from([Argument::from(Term::from(XVar::var("x",Ty::I64)))])},
///     ty:Ty::I64
/// };
/// assert_eq!(call1,call2);
/// assert_eq!(call2,call3)
/// ```
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

/// Create a [`core_lang::syntax::statements::Exit`]
/// if no return type is provided, the type will default to `[core_lang::syntax::types::Ty::I64`]
/// ```
/// use macros::exit;
/// use core_lang::syntax::{types::Ty,terms::xvar::XVar,statements::Exit};
/// let exit1 = exit!(XVar::var("x",Ty::I64),Ty::I64);
/// let exit2 = exit!(XVar::var("x",Ty::I64));
/// let exit3 = Exit::exit(XVar::var("x",Ty::I64),Ty::I64);
/// assert_eq!(exit1,exit2);
/// assert_eq!(exit2,exit3);
/// ```
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

/// Create a [`core_lang::syntax::context::ContextBinding`]
/// If no type is provided, it defaults to [`core_lang::syntax::types::Ty`]
/// ```
/// use macros::bind;
/// use core_lang::syntax::{types::Ty, context::{ContextBinding,Chirality}};
/// let bnd1 = bind!("x",Chirality::Prd);
/// let bnd2 = bind!("x",Chirality::Prd,Ty::I64);
/// let bnd3 = ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64};
/// assert_eq!(bnd1,bnd2);
/// assert_eq!(bnd2,bnd3);
/// ```
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

/// Create a [`core_lang::syntax::def::Def`]
/// ```
/// use macros::def;
/// use core_lang::syntax::{
///     def::Def,
///     types::Ty,
///     context::{Chirality, ContextBinding, TypingContext},
///     statements::{Statement, Call},
///     arguments::{Argument,Arguments},
///     terms::{xvar::XVar, Term}
/// };
/// use std::collections::HashSet;
/// let def1 = def!(
///     "print",
///     [ContextBinding{ var:"x".to_string(), chi: Chirality::Prd, ty: Ty::I64 } ],
///     Call {
///         name:"print_i64".to_string(),
///         args: Arguments { entries: vec![Argument::from(Term::from(XVar::var("x",Ty::I64)))] },
///         ty:Ty::I64
///     }, ["a","x"]);
/// let def2 = Def {
///     name:"print".to_string(),
///     context: TypingContext{
///         bindings: vec![ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64}]
///     },
///     body:Statement::from(Call {
///         name:"print_i64".to_string(),
///         args: Arguments { entries: vec![Argument::from(Term::from(XVar::var("x",Ty::I64)))] },
///         ty:Ty::I64
///     }),
///     used_vars: HashSet::from(["x".to_string(),"a".to_string()])};
/// assert_eq!(def1,def2)
/// ```
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

/// Create a [`core_lang::syntax::program::Program`]
/// ```
/// use macros::prog;
/// use core_lang::syntax::{
///     Data,Codata,
///     declaration::TypeDeclaration,
///     context::TypingContext,
///     def::Def,
///     program::Prog,
///     statements::Exit,
///     terms::XVar,
///     types::Ty,
/// };
/// use std::collections::HashSet;
/// let prog1 = prog!([
///     Def {
///             name:"exit".to_string(),
///             context:TypingContext::default(),
///             body:Exit::exit(XVar::var("x",Ty::I64),Ty::I64),
///             used_vars:HashSet::from(["x".to_string()])
///     }],[
///     TypeDeclaration {
///         dat:Data,
///         name:"Unit".to_string(),
///         xtors:Vec::new()
///     }],[
///     TypeDeclaration {
///         dat:Codata,
///         name:"Void".to_string(),
///         xtors:Vec::new()
///     }]);
/// let prog2 = Prog{
///     defs:vec![
///         Def {
///             name:"exit".to_string(),
///             context:TypingContext::default(),
///             body:Exit::exit(XVar::var("x",Ty::I64),Ty::I64),
///             used_vars:HashSet::from(["x".to_string()])
///         }
///     ],
///     data_types:vec![
///         TypeDeclaration {
///             dat:Data,
///             name:"Unit".to_string(),
///             xtors:Vec::new()
///         }
///     ],
///     codata_types:vec![
///         TypeDeclaration {
///             dat:Codata,
///             name:"Void".to_string(),
///             xtors:Vec::new()
///         }
///     ]
/// };
/// assert_eq!(prog1,prog2)
/// ```
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
