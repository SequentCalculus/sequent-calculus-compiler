use core_lang::syntax::Chirality;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprArray, ExprLit, Lit, Token, parse::Parser, punctuated::Punctuated};

fn parse_args<const N: usize>(input: TokenStream, arg_names: [&str; N]) -> [Expr; N] {
    let mut parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(input.into())
        .expect("Macro arguments could not be parsed")
        .into_iter();
    arg_names.map(|arg_name| {
        let err_msg = format!("Please provide {arg_name}");
        parsed.next().expect(&err_msg)
    })
}

fn expr_to_str(expr: &Expr) -> String {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => s.value(),
        _ => panic!("Please provide string literal"),
    }
}

fn expr_to_array(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::Array(ExprArray { elems, .. }) => elems.into_iter().cloned().collect(),
        _ => panic!("Please provide an array expression"),
    }
}

fn xtor(input: TokenStream, prdcns: Chirality) -> TokenStream {
    let (chi, xtor_desc) = match prdcns {
        Chirality::Prd => (quote! {core_lang::syntax::Prd}, "Ctor Name"),
        Chirality::Cns => (quote! { core_lang::syntax::Cns}, "Dtor Name"),
    };

    let args = parse_args(input, [xtor_desc, "Argument list", "Type"]);

    let xtor_name = expr_to_str(&args[0]);
    let xtor_args = expr_to_array(&args[1])
        .iter()
        .map(|arg| quote! { core_lang::syntax::terms::Term::from(#arg).into() })
        .collect::<Vec<_>>();
    let ty = &args[2];
    quote! {
        core_lang::syntax::terms::xtor::Xtor{
            prdcns: #chi,
            id: #xtor_name.to_string(),
            args: core_lang::syntax::arguments::Arguments { entries: ::std::vec::Vec::from([
                    #(#xtor_args),*
            ]) },
            ty: #ty
        }
    }
    .into()
}

/// Create a [`core_lang::syntax::statements::Cut`] with given arguments
/// ```
/// use macros::cut;
/// use core_lang::syntax::{statements::Cut, terms::xvar::XVar,types::Ty};
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
            producer: ::std::rc::Rc::new(#prod),
            consumer: ::std::rc::Rc::new(#cons),
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
