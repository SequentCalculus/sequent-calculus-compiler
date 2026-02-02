use core_lang::syntax::Chirality;
use proc_macro::TokenStream;
use quote::quote;

pub(crate) mod arguments;
pub(crate) mod context;
pub(crate) mod declarations;
pub(crate) mod prog;
pub(crate) mod statements;
pub(crate) mod terms;
pub(crate) mod types;
mod utils;
use terms::{fs_xtor, unfocused_xtor, xcase, xvar};

///Create a [`core_lang::syntax::types::Ty`] from a string literal\
///`int` will create [`core_lang::syntax::types::Ty::I64`] anything else will
///create [`core_lang::syntax::types::Ty::Decl`]
///
///```
///use core_lang::syntax::types::Ty;
///use macros::ty;
///let int1 = ty!("int");
///let int2 = Ty::I64;
///assert_eq!(int1, int2);
///let list1 = ty!("ListInt");
///let list2 = Ty::Decl("ListInt".to_string());
///assert_eq!(list1, list2)
///```
#[proc_macro]
pub fn ty(input: TokenStream) -> TokenStream {
    types::ty(input)
}

///Create a [`core_lang::syntax::context::ContextBinding`] If no type is provided,
///it defaults to [`core_lang::syntax::types::Ty`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding},
///    types::Ty,
///};
///use macros::bind;
///let bnd1 = bind!("x", Chirality::Prd);
///let bnd2 = bind!("x", Chirality::Prd, Ty::I64);
///let bnd3 = ContextBinding {
///    var: "x".to_string(),
///    chi: Chirality::Prd,
///    ty: Ty::I64,
///};
///assert_eq!(bnd1, bnd2);
///assert_eq!(bnd2, bnd3);
///```
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    context::bind(input)
}

///Create [`core_lang::syntax::context::Chirality::Cns`]
///
///```
///use core_lang::syntax::context::Chirality;
///use macros::cns;
///
///let cns1 = cns!();
///let cns2 = Chirality::Cns;
///assert_eq!(cns1, cns2)
///```
#[proc_macro]
pub fn cns(_: TokenStream) -> TokenStream {
    quote! {core_lang::syntax::context::Chirality::Cns}.into()
}

///Create [`core_lang::syntax::context::Chirality::Prd`]
///
///```
///use core_lang::syntax::context::Chirality;
///use macros::prd;
///
///let cns1 = prd!();
///let cns2 = Chirality::Prd;
///assert_eq!(cns1, cns2)
///```
#[proc_macro]
pub fn prd(_: TokenStream) -> TokenStream {
    quote! {core_lang::syntax::context::Chirality::Prd}.into()
}

// Terms

///Create a [`core_lang::syntax::terms::xvar::XVar`] with chrality
///[`core_lang::syntax::terms::Prd`] If no type is provided, the variable will
///default to [`core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{terms::xvar::XVar, types::Ty};
///use macros::{ty, var};
///let var1 = var!("x");
///let var2 = var!("x", Ty::I64);
///let var3 = XVar::var("x", Ty::I64);
///assert_eq!(var1, var2);
///assert_eq!(var2, var3);
///```
#[proc_macro]
pub fn var(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::xvar::XVar`] with chirality
///[`core_lang::syntax::terms::Cns`] If no type is provided the covariable will
///default to [`core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{terms::xvar::XVar, types::Ty};
///use macros::covar;
///let covar1 = covar!("a");
///let covar2 = covar!("a", Ty::I64);
///let covar3 = XVar::covar("a", Ty::I64);
///assert_eq!(covar1, covar2);
///assert_eq!(covar2, covar3);
///```
#[proc_macro]
pub fn covar(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::literal::Literal`]
///
///```
///use core_lang::syntax::terms::literal::Literal;
///use macros::lit;
///
///let lit1 = lit!(1);
///let lit2 = Literal { lit: 1 };
///assert_eq!(lit1, lit2)
///```
#[proc_macro]
pub fn lit(input: TokenStream) -> TokenStream {
    terms::lit(input)
}

///Create a [`core_lang::syntax::terms::Xtor`] with chirality
///[`core_lang::syntax::terms::Prd`] (i.e. a constructor)
///
///```
///use core_lang::syntax::{
///    arguments::Arguments,
///    terms::{XVar, Xtor},
///    types::Ty,
///};
///use macros::ctor;
///let ctor1 = ctor!(
///    "Cons",
///    [
///        XVar::var("x", Ty::I64),
///        ctor!("Nil", [], Ty::Decl("ListInt".to_string()))
///    ],
///    Ty::Decl("ListInt".to_string())
///);
///
///let mut arguments = Arguments::default();
///arguments.add_prod(XVar::var("x", Ty::I64));
///arguments.add_prod(Xtor::ctor(
///    "Nil",
///    Arguments::default(),
///    Ty::Decl("ListInt".to_string()),
///));
///let ctor2 = Xtor::ctor("Cons", arguments, Ty::Decl("ListInt".to_string()));
///assert_eq!(ctor1, ctor2)
///```
#[proc_macro]
pub fn ctor(input: TokenStream) -> TokenStream {
    unfocused_xtor(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::Xtor`] with chirality
///[`core_lang::syntax::terms::Cns`] (i.e. a destructor)
///
///```
///use core_lang::syntax::{
///    arguments::Arguments,
///    terms::{XVar, Xtor},
///    types::Ty,
///};
///use macros::dtor;
///let dtor1 = dtor!(
///    "apply",
///    [XVar::var("x", Ty::I64)],
///    Ty::Decl("FunI64I64".to_string())
///);
///
///let mut arguments = Arguments::default();
///arguments.add_prod(XVar::var("x", Ty::I64));
///let dtor2 = Xtor::dtor("apply", arguments, Ty::Decl("FunI64I64".to_string()));
///assert_eq!(dtor1, dtor2)
///```
#[proc_macro]
pub fn dtor(input: TokenStream) -> TokenStream {
    unfocused_xtor(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::xtor::FsXtor`] with chirality
///[`core_lang::syntax::Prd`], that is, a focussed constructor
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    terms::xtor::FsXtor,
///    types::Ty,
///    Prd,
///};
///use macros::fs_ctor;
///let xtor1 = fs_ctor!(
///    "Cons",
///    [
///        ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::I64
///        },
///        ContextBinding {
///            var: "xs".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::Decl("ListInt".to_string())
///        }
///    ],
///    Ty::Decl("ListInt".to_string())
///);
///let xtor2 = FsXtor {
///    prdcns: Prd,
///    id: "Cons".to_string(),
///    args: TypingContext {
///        bindings: vec![
///            ContextBinding {
///                var: "x".to_string(),
///                chi: Chirality::Prd,
///                ty: Ty::I64,
///            },
///            ContextBinding {
///                var: "xs".to_string(),
///                chi: Chirality::Prd,
///                ty: Ty::Decl("ListInt".to_string()),
///            },
///        ],
///    },
///    ty: Ty::Decl("ListInt".to_string()),
///};
///assert_eq!(xtor1, xtor2);
///```
#[proc_macro]
pub fn fs_ctor(input: TokenStream) -> TokenStream {
    fs_xtor(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::xtor::FsXtor`] with chirality
///[`core_lang::syntax::Cns`], that is, a focussed destructor
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    terms::xtor::FsXtor,
///    types::Ty,
///    Cns,
///};
///use macros::fs_dtor;
///let xtor1 = fs_dtor!(
///    "apply",
///    [ContextBinding {
///        var: "x".to_string(),
///        chi: Chirality::Prd,
///        ty: Ty::I64
///    },],
///    Ty::Decl("FunIntInt".to_string())
///);
///let xtor2 = FsXtor {
///    prdcns: Cns,
///    id: "apply".to_string(),
///    args: TypingContext {
///        bindings: vec![ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::I64,
///        }],
///    },
///    ty: Ty::Decl("FunIntInt".to_string()),
///};
///assert_eq!(xtor1, xtor2);
///```
#[proc_macro]
pub fn fs_dtor(input: TokenStream) -> TokenStream {
    fs_xtor(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::clause::Clause`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::{Exit, Statement},
///    terms::{clause::Clause, xvar::XVar},
///    types::Ty,
///    Prd,
///};
///use macros::clause;
///use std::rc::Rc;
///
///let clause1 = clause!(
///    Prd,
///    "apply",
///    [ContextBinding {
///        var: "x".to_string(),
///        chi: Chirality::Prd,
///        ty: Ty::I64
///    }],
///    Exit::exit(XVar::var("x", Ty::I64), Ty::I64)
///);
///let clause2 = Clause {
///    prdcns: Prd,
///    xtor: "apply".to_string(),
///    context: TypingContext {
///        bindings: vec![ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::I64,
///        }],
///    },
///    body: Rc::new(Statement::from(Exit::exit(
///        XVar::var("x", Ty::I64),
///        Ty::I64,
///    ))),
///};
///assert_eq!(clause1, clause2)
///```
#[proc_macro]
pub fn clause(input: TokenStream) -> TokenStream {
    terms::unfocused_clause(input)
}

///Create a [`core_lang::syntax::terms::clause::FsClause`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::{exit::FsExit, FsStatement},
///    terms::clause::FsClause,
///    types::Ty,
///    Prd,
///};
///use macros::fs_clause;
///use std::rc::Rc;
///
///let clause1 = fs_clause!(
///    Prd,
///    "apply",
///    [
///        ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::I64
///        },
///        ContextBinding {
///            var: "a".to_string(),
///            chi: Chirality::Cns,
///            ty: Ty::I64
///        }
///    ],
///    FsExit::exit("x")
///);
///let clause2 = FsClause {
///    prdcns: Prd,
///    xtor: "apply".to_string(),
///    context: TypingContext {
///        bindings: vec![
///            ContextBinding {
///                var: "x".to_string(),
///                chi: Chirality::Prd,
///                ty: Ty::I64,
///            },
///            ContextBinding {
///                var: "a".to_string(),
///                chi: Chirality::Cns,
///                ty: Ty::I64,
///            },
///        ],
///    },
///    body: Rc::new(FsStatement::from(FsExit::exit("x"))),
///};
///assert_eq!(clause1, clause2);
///```
#[proc_macro]
pub fn fs_clause(input: TokenStream) -> TokenStream {
    terms::fs_clause(input)
}

///Create a [`core_lang::syntax::terms::xcase::XCase`] with chirality
///[`core_lang::syntax::Cns`], i.e. a case expression If the continuation type is
///not specified, it defaults to [`core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::{Exit, Statement},
///    terms::{clause::Clause, xcase::XCase, xvar::XVar},
///    types::Ty,
///    Cns,
///};
///use macros::case;
///use std::rc::Rc;
///
///let case1 = case!(
///    [
///        Clause {
///            prdcns: Cns,
///            xtor: "Nil".to_string(),
///            context: TypingContext::default(),
///            body: Rc::new(Statement::from(Exit::exit(
///                XVar::var("x", Ty::I64),
///                Ty::I64
///            )))
///        },
///        Clause {
///            prdcns: Cns,
///            xtor: "Cons".to_string(),
///            context: TypingContext {
///                bindings: vec![
///                    ContextBinding {
///                        var: "x".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::I64
///                    },
///                    ContextBinding {
///                        var: "xs".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::Decl("ListInt".to_string())
///                    },
///                    ContextBinding {
///                        var: "a".to_string(),
///                        chi: Chirality::Cns,
///                        ty: Ty::Decl("ListInt".to_string())
///                    }
///                ]
///            },
///            body: Rc::new(Statement::from(Exit::exit(
///                XVar::var("x", Ty::I64),
///                Ty::I64
///            )))
///        }
///    ],
///    Ty::Decl("ListInt".to_string())
///);
///
///let case2 = XCase {
///    prdcns: Cns,
///    clauses: vec![
///        Clause {
///            prdcns: Cns,
///            xtor: "Nil".to_string(),
///            context: TypingContext::default(),
///            body: Rc::new(Statement::from(Exit::exit(
///                XVar::var("x", Ty::I64),
///                Ty::I64,
///            ))),
///        },
///        Clause {
///            prdcns: Cns,
///            xtor: "Cons".to_string(),
///            context: TypingContext {
///                bindings: vec![
///                    ContextBinding {
///                        var: "x".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::I64,
///                    },
///                    ContextBinding {
///                        var: "xs".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::Decl("ListInt".to_string()),
///                    },
///                    ContextBinding {
///                        var: "a".to_string(),
///                        chi: Chirality::Cns,
///                        ty: Ty::Decl("ListInt".to_string()),
///                    },
///                ],
///            },
///            body: Rc::new(Statement::from(Exit::exit(
///                XVar::var("x", Ty::I64),
///                Ty::I64,
///            ))),
///        },
///    ],
///    ty: Ty::Decl("ListInt".to_string()),
///};
///assert_eq!(case1, case2)
///```
#[proc_macro]
pub fn case(input: TokenStream) -> TokenStream {
    xcase(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::xcase::XCase`] with chirality
///[`core_lang::syntax::Cns`] i.e. a cocase / new expression if the return type is
///not specified it defaults to [`core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::exit::Exit,
///    terms::{clause::Clause, xcase::XCase, xvar::XVar},
///    types::Ty,
///    Cns, Prd,
///};
///use macros::cocase;
///use std::rc::Rc;
///
///let cocase1 = cocase!(
///    [Clause {
///        prdcns: Prd,
///        xtor: "apply".to_string(),
///        context: TypingContext {
///            bindings: vec![
///                ContextBinding {
///                    var: "x".to_string(),
///                    chi: Chirality::Prd,
///                    ty: Ty::I64
///                },
///                ContextBinding {
///                    var: "a".to_string(),
///                    chi: Chirality::Cns,
///                    ty: Ty::I64
///                }
///            ]
///        },
///        body: Rc::new(Exit::exit(XVar::var("x", Ty::I64), Ty::I64))
///    }],
///    Ty::Decl("FunI64I64".to_string())
///);
///let cocase2 = XCase {
///    prdcns: Prd,
///    clauses: vec![Clause {
///        prdcns: Prd,
///        xtor: "apply".to_string(),
///        context: TypingContext {
///            bindings: vec![
///                ContextBinding {
///                    var: "x".to_string(),
///                    chi: Chirality::Prd,
///                    ty: Ty::I64,
///                },
///                ContextBinding {
///                    var: "a".to_string(),
///                    chi: Chirality::Cns,
///                    ty: Ty::I64,
///                },
///            ],
///        },
///        body: Rc::new(Exit::exit(XVar::var("x", Ty::I64), Ty::I64)),
///    }],
///    ty: Ty::Decl("FunI64I64".to_string()),
///};
///assert_eq!(cocase1, cocase2)
///```
#[proc_macro]
pub fn cocase(input: TokenStream) -> TokenStream {
    xcase(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::mu::Mu`] with chirality
///[`core_lang::syntax::Prd`],\
///that is, a mu-binding If no type is provided, defaults to
///[`core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{
///    statements::{Exit, Statement},
///    terms::{mu::Mu, xvar::XVar},
///    types::Ty,
///    Prd,
///};
///use macros::mu;
///use std::rc::Rc;
///
///let mu1 = mu!("a", Exit::exit(XVar::var("x", Ty::I64), Ty::I64));
///let mu2 = Mu {
///    prdcns: Prd,
///    variable: "a".to_string(),
///    statement: Rc::new(Statement::from(Exit::exit(
///        XVar::var("x", Ty::I64),
///        Ty::I64,
///    ))),
///    ty: Ty::I64,
///};
///assert_eq!(mu1, mu2)
///```
#[proc_macro]
pub fn mu(input: TokenStream) -> TokenStream {
    terms::unfocused_xmu(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::mu::Mu`] with chirality
///[`core_lang::syntax::Cns`],\
///that is, a mu-tilde-binding If no type is provided, defaults to
///[`core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{
///    statements::{Exit, Statement},
///    terms::{mu::Mu, xvar::XVar},
///    types::Ty,
///    Cns,
///};
///use macros::mutilde;
///use std::rc::Rc;
///
///let mu1 = mutilde!("x", Exit::exit(XVar::var("x", Ty::I64), Ty::I64));
///let mu2 = Mu {
///    prdcns: Cns,
///    variable: "x".to_string(),
///    statement: Rc::new(Statement::from(Exit::exit(
///        XVar::var("x", Ty::I64),
///        Ty::I64,
///    ))),
///    ty: Ty::I64,
///};
///assert_eq!(mu1, mu2)
///```
#[proc_macro]
pub fn mutilde(input: TokenStream) -> TokenStream {
    terms::unfocused_xmu(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::mu::FsMu`] with chirality
///[`core_lang::syntax::Prd`] that is, a focused mu binding. If no type is
///provided, [`core_lang::syntax::types::Ty::I64`] is used
///
///```
///use core_lang::syntax::{
///    statements::{FsExit, FsStatement},
///    terms::{mu::FsMu, XVar},
///    types::Ty,
///    Prd,
///};
///use macros::fs_mu;
///use std::rc::Rc;
///
///let mu1 = fs_mu!("a", FsExit::exit("a"));
///let mu2 = FsMu {
///    prdcns: Prd,
///    variable: "a".to_string(),
///    statement: Rc::new(FsStatement::from(FsExit::exit("a"))),
///    ty: Ty::I64,
///};
///assert_eq!(mu1, mu2)
///```
#[proc_macro]
pub fn fs_mu(input: TokenStream) -> TokenStream {
    terms::fs_xmu(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::mu::FsMu`] with chirality
///[`core_lang::syntax::Cns`] that is, a focused mu-tilde binding. If no type is
///provided, [`core_lang::syntax::types::Ty::I64`] is used
///
///```
///use core_lang::syntax::{
///    statements::{exit::FsExit, FsStatement},
///    terms::{mu::FsMu, XVar},
///    types::Ty,
///    Cns,
///};
///use macros::fs_mutilde;
///use std::rc::Rc;
///
///let mu1 = fs_mutilde!("x", FsExit::exit("x"));
///let mu2 = FsMu {
///    prdcns: Cns,
///    variable: "x".to_string(),
///    statement: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    ty: Ty::I64,
///};
///assert_eq!(mu1, mu2)
///```
#[proc_macro]
pub fn fs_mutilde(input: TokenStream) -> TokenStream {
    terms::fs_xmu(input, Chirality::Cns)
}

// Statements

///Create a [`core_lang::syntax::statements::Cut`] with given arguments if no type
///is provided, the cut type will default to [`core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{statements::Cut, terms::xvar::XVar, types::Ty};
///use macros::cut;
///let cut1 = cut!(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64));
///let cut2 = cut!(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64);
///let cut3 = Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64);
///assert_eq!(cut1, cut2);
///assert_eq!(cut2, cut3)
///```
#[proc_macro]
pub fn cut(input: TokenStream) -> TokenStream {
    statements::unfocused_cut(input)
}

///Create a [`core_lang::syntax::statements::cut::FsCut`] if no type is provided,
///default to [`core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{
///    statements::FsCut,
///    terms::{FsTerm, XVar},
///    types::Ty,
///};
///use macros::fs_cut;
///use std::rc::Rc;
///
///let cut1 = fs_cut!(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64));
///let cut2 = FsCut {
///    producer: Rc::new(FsTerm::from(XVar::var("x", Ty::I64))),
///    consumer: Rc::new(FsTerm::from(XVar::covar("a", Ty::I64))),
///    ty: Ty::I64,
///};
///assert_eq!(cut1, cut2)
///```
#[proc_macro]
pub fn fs_cut(input: TokenStream) -> TokenStream {
    statements::fs_cut(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Equal`]. If only one comparison
///argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        Exit, Statement,
///    },
///    terms::{literal::Literal, Term},
///    types::Ty,
///};
///use macros::ife;
///use std::rc::Rc;
///
///let if1 = ife!(
///    Literal::new(1),
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::Equal,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: Some(Rc::new(Term::from(Literal::new(1)))),
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///
///let if1 = ife!(
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::Equal,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: None,
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ife(input: TokenStream) -> TokenStream {
    statements::unfocused_ife(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::NotEqual`]. If only one comparison
///argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        Exit, Statement,
///    },
///    terms::{literal::Literal, Term},
///    types::Ty,
///};
///use macros::ifne;
///use std::rc::Rc;
///
///let if1 = ifne!(
///    Literal::new(1),
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::NotEqual,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: Some(Rc::new(Term::from(Literal::new(1)))),
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///
///let if1 = ifne!(
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::NotEqual,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: None,
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifne(input: TokenStream) -> TokenStream {
    statements::unfocused_ifne(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Less`]. If only one comparison
///argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        Exit, Statement,
///    },
///    terms::{literal::Literal, Term},
///    types::Ty,
///};
///use macros::ifl;
///use std::rc::Rc;
///
///let if1 = ifl!(
///    Literal::new(1),
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::Less,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: Some(Rc::new(Term::from(Literal::new(1)))),
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///
///let if1 = ifl!(
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::Less,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: None,
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifl(input: TokenStream) -> TokenStream {
    statements::unfocused_ifl(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::LessOrEqual`]. If only one
///comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        Exit, Statement,
///    },
///    terms::{literal::Literal, Term},
///    types::Ty,
///};
///use macros::ifle;
///use std::rc::Rc;
///
///let if1 = ifle!(
///    Literal::new(1),
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::LessOrEqual,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: Some(Rc::new(Term::from(Literal::new(1)))),
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///
///let if1 = ifle!(
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::LessOrEqual,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: None,
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifle(input: TokenStream) -> TokenStream {
    statements::unfocused_ifle(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Greater`]. If only one comparison
///argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        Exit, Statement,
///    },
///    terms::{literal::Literal, Term},
///    types::Ty,
///};
///use macros::ifg;
///use std::rc::Rc;
///
///let if1 = ifg!(
///    Literal::new(1),
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::Greater,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: Some(Rc::new(Term::from(Literal::new(1)))),
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///
///let if1 = ifg!(
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::Greater,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: None,
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifg(input: TokenStream) -> TokenStream {
    statements::unfocused_ifg(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::GreaterOrEqual`]. If only one
///comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        Exit, Statement,
///    },
///    terms::{literal::Literal, Term},
///    types::Ty,
///};
///use macros::ifge;
///use std::rc::Rc;
///
///let if1 = ifge!(
///    Literal::new(1),
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::GreaterOrEqual,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: Some(Rc::new(Term::from(Literal::new(1)))),
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///
///let if1 = ifge!(
///    Literal::new(1),
///    Exit::exit(Literal::new(1), Ty::I64),
///    Exit::exit(Literal::new(2), Ty::I64)
///);
///let if2 = IfC {
///    sort: IfSort::GreaterOrEqual,
///    fst: Rc::new(Term::from(Literal::new(1))),
///    snd: None,
///    thenc: Rc::new(Statement::from(Exit::exit(Literal::new(1), Ty::I64))),
///    elsec: Rc::new(Statement::from(Exit::exit(Literal::new(2), Ty::I64))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifge(input: TokenStream) -> TokenStream {
    statements::unfocused_ifge(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Equal`]. If only one comparison
///argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        FsExit, FsStatement,
///    },
///    terms::{FsTerm, Literal},
///    types::Ty,
///};
///use macros::fs_ife;
///use std::rc::Rc;
///
///let if1 = fs_ife!("x", "y", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::Equal,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///
///let if1 = fs_ife!("x", FsExit::exit("x"), FsExit::exit("x"));
///let if2 = IfC {
///    sort: IfSort::Equal,
///    fst: "x".to_string(),
///    snd: None,
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("x"))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn fs_ife(input: TokenStream) -> TokenStream {
    statements::fs_ife(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::NotEqual`]. If only one comparison
///argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        FsExit, FsStatement,
///    },
///    terms::{FsTerm, Literal},
///    types::Ty,
///};
///use macros::fs_ifne;
///use std::rc::Rc;
///
///let if1 = fs_ifne!("x", "y", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::NotEqual,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///
///let if1 = fs_ifne!("x", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::NotEqual,
///    fst: "x".to_string(),
///    snd: None,
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn fs_ifne(input: TokenStream) -> TokenStream {
    statements::fs_ifne(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Less`]. If only one comparison
///argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        FsExit, FsStatement,
///    },
///    terms::{FsTerm, Literal},
///    types::Ty,
///};
///use macros::fs_ifl;
///use std::rc::Rc;
///
///let if1 = fs_ifl!("x", "y", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::Less,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///
///let if1 = fs_ifl!("x", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::Less,
///    fst: "x".to_string(),
///    snd: None,
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn fs_ifl(input: TokenStream) -> TokenStream {
    statements::fs_ifl(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::LessOrEqual`]. If only one
///comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        FsExit, FsStatement,
///    },
///    terms::{FsTerm, Literal},
///    types::Ty,
///};
///use macros::fs_ifle;
///use std::rc::Rc;
///
///let if1 = fs_ifle!("x", "y", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::LessOrEqual,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///
///let if1 = fs_ifle!("x", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::LessOrEqual,
///    fst: "x".to_string(),
///    snd: None,
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn fs_ifle(input: TokenStream) -> TokenStream {
    statements::fs_ifle(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Greater`]. If only one comparison
///argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        FsExit, FsStatement,
///    },
///    terms::{FsTerm, Literal},
///    types::Ty,
///};
///use macros::fs_ifg;
///use std::rc::Rc;
///
///let if1 = fs_ifg!("x", "y", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::Greater,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///
///let if1 = fs_ifg!("x", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::Greater,
///    fst: "x".to_string(),
///    snd: None,
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn fs_ifg(input: TokenStream) -> TokenStream {
    statements::fs_ifg(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::GreaterOrEqual`]. If only one
///comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
///
///```
///use core_lang::syntax::{
///    statements::{
///        ifc::{IfC, IfSort},
///        FsExit, FsStatement,
///    },
///    terms::{FsTerm, Literal},
///    types::Ty,
///};
///use macros::fs_ifge;
///use std::rc::Rc;
///
///let if1 = fs_ifge!("x", "y", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::GreaterOrEqual,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///
///let if1 = fs_ifge!("x", FsExit::exit("x"), FsExit::exit("y"));
///let if2 = IfC {
///    sort: IfSort::GreaterOrEqual,
///    fst: "x".to_string(),
///    snd: None,
///    thenc: Rc::new(FsStatement::from(FsExit::exit("x"))),
///    elsec: Rc::new(FsStatement::from(FsExit::exit("y"))),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn fs_ifge(input: TokenStream) -> TokenStream {
    statements::fs_ifge(input)
}

///Create a [`core_lang::syntax::statements::Call`] if no return type is provided
///it will default to [`core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{
///    arguments::{Argument, Arguments},
///    statements::Call,
///    terms::{xvar::XVar, Term},
///    types::Ty,
///};
///use macros::call;
///let call1 = call!("print", [XVar::var("x", Ty::I64)], Ty::I64);
///let call2 = call!("print", [XVar::var("x", Ty::I64)]);
///let call3 = Call {
///    name: "print".to_string(),
///    args: Arguments {
///        entries: Vec::from([Argument::from(Term::from(XVar::var("x", Ty::I64)))]),
///    },
///    ty: Ty::I64,
///};
///assert_eq!(call1, call2);
///assert_eq!(call2, call3)
///```
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    statements::unfocused_call(input)
}

///Create a [`core_lang::syntax::statements::call::FsCall`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::call::FsCall,
///    types::Ty,
///};
///use macros::fs_call;
///
///let call1 = fs_call!(
///    "exit",
///    [ContextBinding {
///        var: "x".to_string(),
///        chi: Chirality::Prd,
///        ty: Ty::I64
///    }]
///);
///let call2 = FsCall {
///    name: "exit".to_string(),
///    args: TypingContext {
///        bindings: vec![ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::I64,
///        }],
///    },
///};
///assert_eq!(call1, call2)
///```
#[proc_macro]
pub fn fs_call(input: TokenStream) -> TokenStream {
    statements::fs_call(input)
}

///Create a [`core_lang::syntax::statements::Exit`] if no return type is provided,
///the type will default to `[core_lang::syntax::types::Ty::I64`]
///
///```
///use core_lang::syntax::{statements::Exit, terms::xvar::XVar, types::Ty};
///use macros::exit;
///let exit1 = exit!(XVar::var("x", Ty::I64), Ty::I64);
///let exit2 = exit!(XVar::var("x", Ty::I64));
///let exit3 = Exit::exit(XVar::var("x", Ty::I64), Ty::I64);
///assert_eq!(exit1, exit2);
///assert_eq!(exit2, exit3);
///```
#[proc_macro]
pub fn exit(input: TokenStream) -> TokenStream {
    statements::exit(input)
}

///Create a [`core_lang::syntax::statements::FsExit`]
///
///```
///use core_lang::syntax::statements::FsExit;
///use macros::fs_exit;
///let exit1 = fs_exit!("x");
///let exit2 = FsExit {
///    var: "x".to_string(),
///};
///assert_eq!(exit1, exit2);
///```
#[proc_macro]
pub fn fs_exit(input: TokenStream) -> TokenStream {
    statements::fs_exit(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with
///[`core_lang::syntax::terms::op::BinOp::Div`]
///
///```
///use core_lang::syntax::terms::{
///    literal::Literal,
///    op::{BinOp, Op},
///    Term,
///};
///use macros::div;
///use std::rc::Rc;
///
///let div1 = div!(Literal::new(1), Literal::new(2));
///let div2 = Op {
///    fst: Rc::new(Term::from(Literal::new(1))),
///    op: BinOp::Div,
///    snd: Rc::new(Term::from(Literal::new(2))),
///};
///assert_eq!(div1, div2);
///```
#[proc_macro]
pub fn div(input: TokenStream) -> TokenStream {
    terms::unfocused_div(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with
///[`core_lang::syntax::terms::op::BinOp::Prod`]
///
///```
///use core_lang::syntax::terms::{
///    literal::Literal,
///    op::{BinOp, Op},
///    Term,
///};
///use macros::prod;
///use std::rc::Rc;
///
///let prod1 = prod!(Literal::new(1), Literal::new(2));
///let prod2 = Op {
///    fst: Rc::new(Term::from(Literal::new(1))),
///    op: BinOp::Prod,
///    snd: Rc::new(Term::from(Literal::new(2))),
///};
///assert_eq!(prod1, prod2)
///```
#[proc_macro]
pub fn prod(input: TokenStream) -> TokenStream {
    terms::unfocused_prod(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with
///[`core_lang::syntax::terms::op::BinOp::Rem`]
///
///```
///use core_lang::syntax::terms::{
///    literal::Literal,
///    op::{BinOp, Op},
///    Term,
///};
///use macros::rem;
///use std::rc::Rc;
///
///let rem1 = rem!(Literal::new(1), Literal::new(2));
///let rem2 = Op {
///    fst: Rc::new(Term::from(Literal::new(1))),
///    op: BinOp::Rem,
///    snd: Rc::new(Term::from(Literal::new(2))),
///};
///assert_eq!(rem1, rem2)
///```
#[proc_macro]
pub fn rem(input: TokenStream) -> TokenStream {
    terms::unfocused_rem(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with
///[`core_lang::syntax::terms::op::BinOp::Sub`]
///
///```
///use core_lang::syntax::terms::{
///    literal::Literal,
///    op::{BinOp, Op},
///    Term,
///};
///use macros::sub;
///use std::rc::Rc;
///
///let sub1 = sub!(Literal::new(1), Literal::new(2));
///let sub2 = Op {
///    fst: Rc::new(Term::from(Literal::new(1))),
///    op: BinOp::Sub,
///    snd: Rc::new(Term::from(Literal::new(2))),
///};
///assert_eq!(sub1, sub2)
///```
#[proc_macro]
pub fn sub(input: TokenStream) -> TokenStream {
    terms::unfocused_sub(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with
///[`core_lang::syntax::terms::op::BinOp::Sum`]
///
///```
///use core_lang::syntax::terms::{
///    literal::Literal,
///    op::{BinOp, Op},
///    Term,
///};
///use macros::sum;
///use std::rc::Rc;
///
///let sum1 = sum!(Literal::new(1), Literal::new(2));
///let sum2 = Op {
///    fst: Rc::new(Term::from(Literal::new(1))),
///    op: BinOp::Sum,
///    snd: Rc::new(Term::from(Literal::new(2))),
///};
///assert_eq!(sum1, sum2)
///```
#[proc_macro]
pub fn sum(input: TokenStream) -> TokenStream {
    terms::unfocused_sum(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with
///[`core_lang::syntax::terms::op::BinOp::Div`], that is a focused divistion term.
///
///```
///use core_lang::syntax::terms::op::{BinOp, FsOp};
///use macros::fs_div;
///let div1 = fs_div!("x", "y");
///let div2 = FsOp {
///    fst: "x".to_string(),
///    op: BinOp::Div,
///    snd: "y".to_string(),
///};
///assert_eq!(div1, div2)
///```
#[proc_macro]
pub fn fs_div(input: TokenStream) -> TokenStream {
    terms::fs_div(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with
///[`core_lang::syntax::terms::op::BinOp::Prod`], that is a focused divistion term.
///
///```
///use core_lang::syntax::terms::op::{BinOp, FsOp};
///use macros::fs_prod;
///let prod1 = fs_prod!("x", "y");
///let prod2 = FsOp {
///    fst: "x".to_string(),
///    op: BinOp::Prod,
///    snd: "y".to_string(),
///};
///assert_eq!(prod1, prod2)
///```
#[proc_macro]
pub fn fs_prod(input: TokenStream) -> TokenStream {
    terms::fs_prod(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with
///[`core_lang::syntax::terms::op::BinOp::Rem`], that is a focused divistion term.
///
///```
///use core_lang::syntax::terms::op::{BinOp, FsOp};
///use macros::fs_rem;
///let rem1 = fs_rem!("x", "y");
///let rem2 = FsOp {
///    fst: "x".to_string(),
///    op: BinOp::Rem,
///    snd: "y".to_string(),
///};
///assert_eq!(rem1, rem2)
///```
#[proc_macro]
pub fn fs_rem(input: TokenStream) -> TokenStream {
    terms::fs_rem(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with
///[`core_lang::syntax::terms::op::BinOp::Sub`], that is a focused divistion term.
///
///```
///use core_lang::syntax::terms::op::{BinOp, FsOp};
///use macros::fs_sub;
///let sub1 = fs_sub!("x", "y");
///let sub2 = FsOp {
///    fst: "x".to_string(),
///    op: BinOp::Sub,
///    snd: "y".to_string(),
///};
///assert_eq!(sub1, sub2)
///```
#[proc_macro]
pub fn fs_sub(input: TokenStream) -> TokenStream {
    terms::fs_sub(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with
///[`core_lang::syntax::terms::op::BinOp::Sum`], that is a focused divistion term.
///
///```
///use core_lang::syntax::terms::op::{BinOp, FsOp};
///use macros::fs_sum;
///let sum1 = fs_sum!("x", "y");
///let sum2 = FsOp {
///    fst: "x".to_string(),
///    op: BinOp::Sum,
///    snd: "y".to_string(),
///};
///assert_eq!(sum1, sum2)
///```
#[proc_macro]
pub fn fs_sum(input: TokenStream) -> TokenStream {
    terms::fs_sum(input)
}

// Declarations

///Create a [`core_lang::syntax::def::Def`]
///
///```
///use core_lang::syntax::{
///    arguments::{Argument, Arguments},
///    context::{Chirality, ContextBinding, TypingContext},
///    def::Def,
///    statements::{Call, Statement},
///    terms::{xvar::XVar, Term},
///    types::Ty,
///};
///use macros::def;
///use std::collections::HashSet;
///let def1 = def!(
///    "print",
///    [ContextBinding {
///        var: "x".to_string(),
///        chi: Chirality::Prd,
///        ty: Ty::I64
///    }],
///    Call {
///        name: "print_i64".to_string(),
///        args: Arguments {
///            entries: vec![Argument::from(Term::from(XVar::var("x", Ty::I64)))]
///        },
///        ty: Ty::I64
///    },
///    ["a", "x"]
///);
///let def2 = Def {
///    name: "print".to_string(),
///    context: TypingContext {
///        bindings: vec![ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::I64,
///        }],
///    },
///    body: Statement::from(Call {
///        name: "print_i64".to_string(),
///        args: Arguments {
///            entries: vec![Argument::from(Term::from(XVar::var("x", Ty::I64)))],
///        },
///        ty: Ty::I64,
///    }),
///    used_vars: HashSet::from(["x".to_string(), "a".to_string()]),
///};
///assert_eq!(def1, def2)
///```
#[proc_macro]
pub fn def(input: TokenStream) -> TokenStream {
    declarations::unfocused_def(input)
}

///Create a [`core_lang::syntax::def::FsDef`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    def::FsDef,
///    statements::{exit::FsExit, FsStatement},
///    types::Ty,
///};
///use macros::fs_def;
///use std::{collections::HashSet, rc::Rc};
///
///let def1 = fs_def!(
///    "exit",
///    [ContextBinding {
///        var: "x".to_string(),
///        chi: Chirality::Prd,
///        ty: Ty::I64
///    }],
///    FsExit::exit("x"),
///    ["x"]
///);
///let def2 = FsDef {
///    name: "exit".to_string(),
///    context: TypingContext {
///        bindings: vec![ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::I64,
///        }],
///    },
///    body: FsStatement::from(FsExit::exit("x")),
///    used_vars: HashSet::from(["x".to_string()]),
///};
///```
#[proc_macro]
pub fn fs_def(input: TokenStream) -> TokenStream {
    declarations::fs_def(input)
}

///Create a [`core_lang::syntax::declaration::DataDeclaration`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    declaration::{Data, DataDeclaration, XtorSig},
///    types::Ty,
///};
///use macros::data;
///let decl1 = data!(
///    "List",
///    [
///        XtorSig {
///            xtor: Data,
///            name: "Nil".to_string(),
///            args: TypingContext { bindings: vec![] }
///        },
///        XtorSig {
///            xtor: Data,
///            name: "Cons".to_string(),
///            args: TypingContext {
///                bindings: vec![
///                    ContextBinding {
///                        var: "x".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::I64
///                    },
///                    ContextBinding {
///                        var: "xs".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::Decl("ListInt".to_string())
///                    },
///                ]
///            }
///        }
///    ]
///);
///let decl2 = DataDeclaration {
///    dat: Data,
///    name: "List".to_string(),
///    xtors: vec![
///        XtorSig {
///            xtor: Data,
///            name: "Nil".to_string(),
///            args: TypingContext { bindings: vec![] },
///        },
///        XtorSig {
///            xtor: Data,
///            name: "Cons".to_string(),
///            args: TypingContext {
///                bindings: vec![
///                    ContextBinding {
///                        var: "x".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::I64,
///                    },
///                    ContextBinding {
///                        var: "xs".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::Decl("ListInt".to_string()),
///                    },
///                ],
///            },
///        },
///    ],
///};
///assert_eq!(decl1, decl2);
///```
#[proc_macro]
pub fn data(input: TokenStream) -> TokenStream {
    declarations::data(input)
}

///Create a [`core_lang::syntax::declaration::CodataDeclaration`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    declaration::{Codata, CodataDeclaration, XtorSig},
///    types::Ty,
///};
///use macros::codata;
///let codata1 = codata!(
///    "FunIntInt",
///    [XtorSig {
///        xtor: Codata,
///        name: "apply".to_string(),
///        args: TypingContext {
///            bindings: vec![
///                ContextBinding {
///                    var: "x".to_string(),
///                    chi: Chirality::Prd,
///                    ty: Ty::I64
///                },
///                ContextBinding {
///                    var: "a".to_string(),
///                    chi: Chirality::Cns,
///                    ty: Ty::I64
///                },
///            ]
///        }
///    }]
///);
///let codata2 = CodataDeclaration {
///    dat: Codata,
///    name: "FunIntInt".to_string(),
///    xtors: vec![XtorSig {
///        xtor: Codata,
///        name: "apply".to_string(),
///        args: TypingContext {
///            bindings: vec![
///                ContextBinding {
///                    var: "x".to_string(),
///                    chi: Chirality::Prd,
///                    ty: Ty::I64,
///                },
///                ContextBinding {
///                    var: "a".to_string(),
///                    chi: Chirality::Cns,
///                    ty: Ty::I64,
///                },
///            ],
///        },
///    }],
///};
///assert_eq!(codata1, codata2);
///```
#[proc_macro]
pub fn codata(input: TokenStream) -> TokenStream {
    declarations::codata(input)
}

///Create a [`core_lang::syntax::declaration::CtorSig`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    declaration::{CtorSig, Data},
///    types::Ty,
///};
///use macros::ctor_sig;
///let ctor1 = ctor_sig!(
///    "Cons",
///    [
///        ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::I64
///        },
///        ContextBinding {
///            var: "xs".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::Decl("ListInt".to_string())
///        }
///    ]
///);
///let ctor2 = CtorSig {
///    xtor: Data,
///    name: "Cons".to_string(),
///    args: TypingContext {
///        bindings: vec![
///            ContextBinding {
///                var: "x".to_string(),
///                chi: Chirality::Prd,
///                ty: Ty::I64,
///            },
///            ContextBinding {
///                var: "xs".to_string(),
///                chi: Chirality::Prd,
///                ty: Ty::Decl("ListInt".to_string()),
///            },
///        ],
///    },
///};
///assert_eq!(ctor1, ctor2)
///```
#[proc_macro]
pub fn ctor_sig(input: TokenStream) -> TokenStream {
    declarations::ctor_sig(input)
}

///Create a [`core_lang::syntax::declaration::DtorSig`]
///
///```
///use core_lang::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    declaration::{Codata, DtorSig},
///    types::Ty,
///};
///use macros::dtor_sig;
///
///let dtor1 = dtor_sig!(
///    "apply",
///    [
///        ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::I64
///        },
///        ContextBinding {
///            var: "a".to_string(),
///            chi: Chirality::Cns,
///            ty: Ty::I64
///        }
///    ]
///);
///let dtor2 = DtorSig {
///    xtor: Codata,
///    name: "apply".to_string(),
///    args: TypingContext {
///        bindings: vec![
///            ContextBinding {
///                var: "x".to_string(),
///                chi: Chirality::Prd,
///                ty: Ty::I64,
///            },
///            ContextBinding {
///                var: "a".to_string(),
///                chi: Chirality::Cns,
///                ty: Ty::I64,
///            },
///        ],
///    },
///};
///assert_eq!(dtor1, dtor2)
///```
#[proc_macro]
pub fn dtor_sig(input: TokenStream) -> TokenStream {
    declarations::dtor_sig(input)
}

///Create a [`core_lang::syntax::program::Prog`] ///
///
///```
///use core_lang::syntax::{
///    context::TypingContext, declaration::TypeDeclaration, def::Def, program::Prog,
///    statements::Exit, terms::XVar, types::Ty, Codata, Data,
///};
///use macros::prog;
///use std::collections::HashSet;
///let prog1 = prog!(
///    [Def {
///        name: "exit".to_string(),
///        context: TypingContext::default(),
///        body: Exit::exit(XVar::var("x", Ty::I64), Ty::I64),
///        used_vars: HashSet::from(["x".to_string()])
///    }],
///    [TypeDeclaration {
///        dat: Data,
///        name: "Unit".to_string(),
///        xtors: Vec::new()
///    }],
///    [TypeDeclaration {
///        dat: Codata,
///        name: "Void".to_string(),
///        xtors: Vec::new()
///    }]
///);
///let prog2 = Prog {
///    defs: vec![Def {
///        name: "exit".to_string(),
///        context: TypingContext::default(),
///        body: Exit::exit(XVar::var("x", Ty::I64), Ty::I64),
///        used_vars: HashSet::from(["x".to_string()]),
///    }],
///    data_types: vec![TypeDeclaration {
///        dat: Data,
///        name: "Unit".to_string(),
///        xtors: Vec::new(),
///    }],
///    codata_types: vec![TypeDeclaration {
///        dat: Codata,
///        name: "Void".to_string(),
///        xtors: Vec::new(),
///    }],
///};
///assert_eq!(prog1, prog2)
///```
#[proc_macro]
pub fn prog(input: TokenStream) -> TokenStream {
    prog::prog(input)
}
