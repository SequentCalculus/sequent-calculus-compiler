use core_lang::syntax::Chirality;
use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

mod utils;
use utils::{expr_to_array, expr_to_str, parse_args};

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
    let args = parse_args(input, ["Type Name"]);
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

    let args = parse_args(input, [xtor_desc, "Argument list", "Type"]);

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

/// Create a [`core_lang::syntax::statements::Cut`] with given arguments
/// ```
/// use macros::cut;
/// use core_lang::syntax::{ statements::Cut, terms::xvar::XVar,types::Ty};
/// let cut1 = cut!(XVar::var("x",Ty::I64),XVar::covar("a",Ty::I64),Ty::I64);
/// let cut2 = Cut::new(XVar::var("x",Ty::I64),XVar::covar("a",Ty::I64),Ty::I64);
/// assert_eq!(cut1,cut2)
/// ```
#[proc_macro]
pub fn cut(input: TokenStream) -> TokenStream {
    let args = parse_args(input, ["Producer", "Consumer", "Type"]);
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

/// Create a [`core_lang::syntax::terms::Xtor`] with chirality [`core_lang::syntax::Chirality::Prd`]
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

/// Create a [`core_lang::syntax::terms::Xtor`] with chirality [`core_lang::syntax::Chirality::Cns`]
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
        [
            "If Sort",
            "If first",
            "If Second",
            "Then Clause",
            "Else Clause",
        ],
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
    let args = parse_args(input, ["If Sort", "If First", "Then Clause", "Else Clause"]);
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
/// ```
/// use macros::call;
/// use core_lang::syntax::{arguments::{Argument,Arguments}, statements::Call, types::Ty, terms::{Term, xvar::XVar}};
/// let call1 = call!("print",[XVar::var("x",Ty::I64)],Ty::I64);
/// let call2 = Call{
///     name:"print".to_string(),
///     args:Arguments{entries:Vec::from([Argument::from(Term::from(XVar::var("x",Ty::I64)))])},
///     ty:Ty::I64
/// };
/// assert_eq!(call1,call2)
/// ```
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    let args = parse_args(input, ["Called Name", "Arguments", "Return Type"]);
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
