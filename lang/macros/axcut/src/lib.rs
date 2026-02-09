use proc_macro::TokenStream;
use quote::quote;

pub(crate) mod context;
pub(crate) mod declarations;
pub(crate) mod program;
pub(crate) mod statements;
pub(crate) mod types;

///Create a [`axcut::syntax::types::Type`] from a string literal\
///`int` will create [`axcut::syntax::types::Type::I64`] anything else will
///create [`axcut::syntax::types::Type::Decl`]
///
///```
///use axcut_macros::ty;
///use axcut::syntax::types::Ty;
///let int1 = ty!("int");
///let int2 = Ty::I64;
///assert_eq!(int1,int2);
///let list1 = ty!("ListInt");
///let list2 = Ty::Decl("ListInt".to_string());
///assert_eq!(list1,list2)
///```
#[proc_macro]
pub fn ty(input: TokenStream) -> TokenStream {
    types::ty(input)
}

///Create a [`axcut::syntax::context::ContextBinding`] If no type is provided,
///it defaults to [`axcut::syntax::types::Ty`]
///
///```
///use axcut_macros::bind;
///use axcut::syntax::{types::Ty, context::{ContextBinding,Chirality}};
///let bnd1 = bind!("x",Chirality::Prd);
///let bnd2 = bind!("x",Chirality::Prd,Ty::I64);
///let bnd3 = ContextBinding{var:"x".to_string(),chi:Chirality::Prd,ty:Ty::I64};
///assert_eq!(bnd1,bnd2);
///assert_eq!(bnd2,bnd3);
///```
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    context::bind(input)
}

///Create [`axcut::syntax::context::Chirality::Prd`]
///
///```
///use axcut::syntax::context::Chirality;
///use axcut_macros::prd;
///
///let cns1 = prd!();
///let cns2 = Chirality::Prd;
///assert_eq!(cns1, cns2)
///```
#[proc_macro]
pub fn prd(_: TokenStream) -> TokenStream {
    quote! {axcut::syntax::context::Chirality::Prd}.into()
}

///Create [`axcut::syntax::context::Chirality::Cns`]
///
///```
///use axcut::syntax::context::Chirality;
///use axcut_macros::cns;
///
///let cns1 = cns!();
///let cns2 = Chirality::Cns;
///assert_eq!(cns1, cns2)
///```
#[proc_macro]
pub fn cns(_: TokenStream) -> TokenStream {
    quote! {axcut::syntax::context::Chirality::Cns}.into()
}

///Create [`axcut::syntax::context::Chirality::Ext`]
///
///```
///use axcut::syntax::context::Chirality;
///use axcut_macros::ext;
///let ext1 = ext!();
///let ext2 = Chirality::Ext;
///assert_eq!(ext1, ext2)
///```
#[proc_macro]
pub fn ext(_: TokenStream) -> TokenStream {
    quote! {axcut::syntax::context::Chirality::Ext}.into()
}

// Statements
///Create a [`axcut::syntax::statements::substitute::Substitute`]
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding},
///    statements::{exit::Exit, substitute::Substitute, Statement},
///    types::Ty,
///};
///use axcut_macros::substitute;
///use std::rc::Rc;
///
///let subst1 = substitute!(
///    [
///        (
///            ContextBinding {
///                var: "x".to_string(),
///                chi: Chirality::Ext,
///                ty: Ty::I64
///            },
///            "x"
///        ),
///        (
///            ContextBinding {
///                var: "a".to_string(),
///                chi: Chirality::Cns,
///                ty: Ty::Decl("Cont".to_string())
///            },
///            "a"
///        )
///    ],
///    Exit {
///        var: "x".to_string()
///    }
///);
///let subst2 = Substitute {
///    rearrange: vec![
///        (
///            ContextBinding {
///                var: "x".to_string(),
///                chi: Chirality::Ext,
///                ty: Ty::I64,
///            },
///            "x".to_string(),
///        ),
///        (
///            ContextBinding {
///                var: "a".to_string(),
///                chi: Chirality::Cns,
///                ty: Ty::Decl("Cont".to_string()),
///            },
///            "a".to_string(),
///        ),
///    ],
///    next: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///};
///assert_eq!(subst1, subst2)
///```
#[proc_macro]
pub fn substitute(input: TokenStream) -> TokenStream {
    statements::substitute(input)
}

///Create a [`axcut::syntax::statements::let::Let`]. `free_vars_next` and `ty` are
///optional, `free_vars_next` defaults to `None` and `ty` defaults to
///[`axcut::syntax::types::Ty::I64`]
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::{Exit, Let, Statement},
///    types::Ty,
///};
///use axcut_macros::letin;
///use std::{collections::HashSet, rc::Rc};
///
///let let1 = letin!(
///    "x",
///    Ty::I64,
///    "Ret",
///    [ContextBinding {
///        var: "y".to_string(),
///        chi: Chirality::Ext,
///        ty: Ty::I64
///    }],
///    Exit {
///        var: "x".to_string()
///    },
///    ["x"]
///);
///let let2 = Let {
///    var: "x".to_string(),
///    ty: Ty::I64,
///    tag: "Ret".to_string(),
///    args: TypingContext {
///        bindings: vec![ContextBinding {
///            var: "y".to_string(),
///            chi: Chirality::Ext,
///            ty: Ty::I64,
///        }],
///    },
///    next: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["x".to_string()])),
///};
///assert_eq!(let1, let2);
///```
#[proc_macro]
pub fn letin(input: TokenStream) -> TokenStream {
    statements::letin(input)
}

///Create a [`axcut::syntax::statements::Switch`]. `free_vars_clauses` and `ty` are
///optional. `ty` defaults to [`axcut::syntax::types::Ty::I64`] and
///`free_vars_clauses` defaults to `None`. It is not possible to skip `ty` and
///provide `free_vars_clauses`. Instead use `axcut_macros::ty`] with argument
///`"int"`.
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::{Clause, Exit, Statement, Switch},
///    types::Ty,
///};
///use axcut_macros::switch;
///use std::{collections::HashSet, rc::Rc};
///
///let switch1 = switch!(
///    "x",
///    Ty::Decl("ListInt".to_string()),
///    [
///        Clause {
///            xtor: "Nil".to_string(),
///            context: TypingContext { bindings: vec![] },
///            body: Rc::new(Statement::from(Exit {
///                var: "x".to_string()
///            }))
///        },
///        Clause {
///            xtor: "Cons".to_string(),
///            context: TypingContext {
///                bindings: vec![
///                    ContextBinding {
///                        var: "x".to_string(),
///                        chi: Chirality::Ext,
///                        ty: Ty::I64
///                    },
///                    ContextBinding {
///                        var: "xs".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::Decl("ListInt".to_string())
///                    }
///                ]
///            },
///            body: Rc::new(Statement::from(Exit {
///                var: "x".to_string()
///            }))
///        }
///    ],
///    ["x"]
///);
///let switch2 = Switch {
///    var: "x".to_string(),
///    ty: Ty::Decl("ListInt".to_string()),
///    clauses: vec![
///        Clause {
///            xtor: "Nil".to_string(),
///            context: TypingContext { bindings: vec![] },
///            body: Rc::new(Statement::from(Exit {
///                var: "x".to_string(),
///            })),
///        },
///        Clause {
///            xtor: "Cons".to_string(),
///            context: TypingContext {
///                bindings: vec![
///                    ContextBinding {
///                        var: "x".to_string(),
///                        chi: Chirality::Ext,
///                        ty: Ty::I64,
///                    },
///                    ContextBinding {
///                        var: "xs".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::Decl("ListInt".to_string()),
///                    },
///                ],
///            },
///            body: Rc::new(Statement::from(Exit {
///                var: "x".to_string(),
///            })),
///        },
///    ],
///    free_vars_clauses: Some(HashSet::from(["x".to_string()])),
///};
///assert_eq!(switch1, switch2);
///```
#[proc_macro]
pub fn switch(input: TokenStream) -> TokenStream {
    statements::switch(input)
}

///Create a [`axcut::syntax::statements::invoke::Invoke`]. If no type is provided,
///defaults to [`axcut::syntax::types::Ty::I64`]
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::invoke::Invoke,
///    types::Ty,
///};
///use axcut_macros::invoke;
///let invoke1 = invoke!(
///    "f",
///    "apply",
///    Ty::Decl("FunIntInt".to_string()),
///    [ContextBinding {
///        var: "x".to_string(),
///        chi: Chirality::Ext,
///        ty: Ty::I64
///    }],
///);
///let invoke2 = Invoke {
///    var: "f".to_string(),
///    tag: "apply".to_string(),
///    ty: Ty::Decl("FunIntInt".to_string()),
///    args: TypingContext {
///        bindings: vec![ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Ext,
///            ty: Ty::I64,
///        }],
///    },
///};
///assert_eq!(invoke1, invoke2);
///```
#[proc_macro]
pub fn invoke(input: TokenStream) -> TokenStream {
    statements::invoke(input)
}

///Create a [`axcut::syntax::statements::create::Create`]. `context`,
///`free_vars_clauses` and `free_vars_next` are optional and default to `None` if
///not provided. However, since they are parsed in order, if `context` is `None`
///but `free_vars_clauses` is `Some`, `context` has to be provided (as `None`)
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::{clause::Clause, create::Create, exit::Exit, Statement},
///    types::Ty,
///};
///use axcut_macros::create;
///use std::{collections::HashSet, rc::Rc};
///
///let create1 = create!(
///    "x",
///    Ty::Decl("FunIntInt".to_string()),
///    [ContextBinding {
///        var: "x".to_string(),
///        chi: Chirality::Ext,
///        ty: Ty::I64
///    }],
///    [Clause {
///        xtor: "apply".to_string(),
///        context: TypingContext {
///            bindings: vec![
///                ContextBinding {
///                    var: "y".to_string(),
///                    chi: Chirality::Ext,
///                    ty: Ty::I64
///                },
///                ContextBinding {
///                    var: "a".to_string(),
///                    chi: Chirality::Cns,
///                    ty: Ty::Decl("Cont".to_string())
///                }
///            ]
///        },
///        body: Rc::new(Statement::from(Exit {
///            var: "y".to_string()
///        }))
///    }],
///    ["y", "a"],
///    Exit {
///        var: "x".to_string()
///    },
///    ["x"]
///);
///let create2 = Create {
///    var: "x".to_string(),
///    ty: Ty::Decl("FunIntInt".to_string()),
///    context: Some(TypingContext {
///        bindings: vec![ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Ext,
///            ty: Ty::I64,
///        }],
///    }),
///    clauses: vec![Clause {
///        xtor: "apply".to_string(),
///        context: TypingContext {
///            bindings: vec![
///                ContextBinding {
///                    var: "y".to_string(),
///                    chi: Chirality::Ext,
///                    ty: Ty::I64,
///                },
///                ContextBinding {
///                    var: "a".to_string(),
///                    chi: Chirality::Cns,
///                    ty: Ty::Decl("Cont".to_string()),
///                },
///            ],
///        },
///        body: Rc::new(Statement::from(Exit {
///            var: "y".to_string(),
///        })),
///    }],
///    free_vars_clauses: Some(HashSet::from(["y".to_string(), "a".to_string()])),
///    next: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["x".to_string()])),
///};
///assert_eq!(create1, create2);
///
///let create1 = create!(
///    "x",
///    Ty::Decl("Cont".to_string()),
///    [ContextBinding {
///        var: "y".to_string(),
///        chi: Chirality::Ext,
///        ty: Ty::I64
///    }],
///    [Clause {
///        xtor: "Cont".to_string(),
///        context: TypingContext { bindings: vec![] },
///        body: Rc::new(Statement::from(Exit {
///            var: "y".to_string()
///        }))
///    }],
///    Exit {
///        var: "x".to_string()
///    }
///);
///let create2 = Create {
///    var: "x".to_string(),
///    ty: Ty::Decl("Cont".to_string()),
///    context: Some(TypingContext {
///        bindings: vec![ContextBinding {
///            var: "y".to_string(),
///            chi: Chirality::Ext,
///            ty: Ty::I64,
///        }],
///    }),
///    clauses: vec![Clause {
///        xtor: "Cont".to_string(),
///        context: TypingContext { bindings: vec![] },
///        body: Rc::new(Statement::from(Exit {
///            var: "y".to_string(),
///        })),
///    }],
///    free_vars_clauses: None,
///    next: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    free_vars_next: None,
///};
///assert_eq!(create1, create2);
///```
#[proc_macro]
pub fn create(input: TokenStream) -> TokenStream {
    statements::create(input)
}

///Create a [`axcut::syntax::statements::Literal`]. If `free_vars_next` is not
///provided, default to `None`
///
///```
///use axcut::syntax::statements::{literal::Literal, Exit, Statement};
///use axcut_macros::lit;
///use std::{collections::HashSet, rc::Rc};
///
///let lit1 = lit!(
///    1,
///    "x",
///    Exit {
///        var: "x".to_string()
///    },
///    ["x"]
///);
///let lit2 = Literal {
///    lit: 1,
///    var: "x".to_string(),
///    next: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["x".to_string()])),
///};
///assert_eq!(lit1, lit2);
///```
#[proc_macro]
pub fn lit(input: TokenStream) -> TokenStream {
    statements::lit(input)
}

///Create a [`axcut::syntax::statements::call::Call`]
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::call::Call,
///    types::Ty,
///};
///use axcut_macros::call;
///
///let call1 = call!(
///    "exit",
///    [ContextBinding {
///        var: "x".to_string(),
///        chi: Chirality::Ext,
///        ty: Ty::I64
///    }]
///);
///let call2 = Call {
///    label: "exit".to_string(),
///    args: TypingContext {
///        bindings: vec![ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Ext,
///            ty: Ty::I64,
///        }],
///    },
///};
///assert_eq!(call1, call2)
///```
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    statements::call(input)
}

///Create a [`axcut::syntax::statements::print::PrintI64`] without newline.
///`free_vars_next` is optional and defaults to `None`
///
///```
///use axcut::syntax::statements::{exit::Exit, print::PrintI64, Statement};
///use axcut_macros::print_i64;
///use std::{collections::HashSet, rc::Rc};
///
///let print1 = print_i64!(
///    "x",
///    Exit {
///        var: "x".to_string()
///    },
///    ["x"]
///);
///let print2 = PrintI64 {
///    newline: false,
///    var: "x".to_string(),
///    next: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["x".to_string()])),
///};
///assert_eq!(print1, print2)
///```
#[proc_macro]
pub fn print_i64(input: TokenStream) -> TokenStream {
    statements::print_i64(input)
}

///Create a [`axcut::syntax::statements::print::PrintI64`] with newline
///`free_vars_next` is optional and defaults to `None`
///
///```
///use axcut::syntax::statements::{exit::Exit, print::PrintI64, Statement};
///use axcut_macros::println_i64;
///use std::{collections::HashSet, rc::Rc};
///
///let print1 = println_i64!(
///    "x",
///    Exit {
///        var: "x".to_string()
///    },
///    ["x".to_string()]
///);
///let print2 = PrintI64 {
///    newline: true,
///    var: "x".to_string(),
///    next: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["x".to_string()])),
///};
///assert_eq!(print1, print2)
///```
#[proc_macro]
pub fn println_i64(input: TokenStream) -> TokenStream {
    statements::println_i64(input)
}

///Create a [`axcut::syntax::statements::exit::Exit`]
///
///```
///use axcut::syntax::statements::exit::Exit;
///use axcut_macros::exit;
///
///let exit1 = exit!("x");
///let exit2 = Exit {
///    var: "x".to_string(),
///};
///assert_eq!(exit1, exit2)
///```
#[proc_macro]
pub fn exit(input: TokenStream) -> TokenStream {
    statements::exit(input)
}

///Create a [`axcut::syntax::statements::Clause`]
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    statements::{Clause, Exit, Statement},
///    types::Ty,
///};
///use axcut_macros::clause;
///use std::rc::Rc;
///
///let clause1 = clause!(
///    "Cons",
///    [
///        ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Ext,
///            ty: Ty::I64
///        },
///        ContextBinding {
///            var: "xs".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::Decl("ListInt".to_string())
///        }
///    ],
///    Exit {
///        var: "x".to_string()
///    }
///);
///let clause2 = Clause {
///    xtor: "Cons".to_string(),
///    context: TypingContext {
///        bindings: vec![
///            ContextBinding {
///                var: "x".to_string(),
///                chi: Chirality::Ext,
///                ty: Ty::I64,
///            },
///            ContextBinding {
///                var: "xs".to_string(),
///                chi: Chirality::Prd,
///                ty: Ty::Decl("ListInt".to_string()),
///            },
///        ],
///    },
///    body: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///};
///```
#[proc_macro]
pub fn clause(input: TokenStream) -> TokenStream {
    statements::clause(input)
}

///Create a [`axcut::syntax::statements::op::Op`] with
///[`axcut::syntax::statements::op::BinOp::Div`]. If `free_vars_next` is not
///provided, it defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    exit::Exit,
///    op::{BinOp, Op},
///    Statement,
///};
///use axcut_macros::div;
///use std::{collections::HashSet, rc::Rc};
///
///let div1 = div!(
///    "x",
///    "y",
///    "z",
///    Exit {
///        var: "z".to_string()
///    },
///    ["z"]
///);
///let div2 = Op {
///    fst: "x".to_string(),
///    op: BinOp::Div,
///    snd: "y".to_string(),
///    var: "z".to_string(),
///    next: Rc::new(Statement::from(Exit {
///        var: "z".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["z".to_string()])),
///};
///assert_eq!(div1, div2)
///```
#[proc_macro]
pub fn div(input: TokenStream) -> TokenStream {
    statements::div(input)
}

///Create a [`axcut::syntax::statements::op::Op`] with
///[`axcut::syntax::statements::op::BinOp::Prod`]. If `free_vars_next` is not
///provided, it defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    exit::Exit,
///    op::{BinOp, Op},
///    Statement,
///};
///use axcut_macros::prod;
///use std::{collections::HashSet, rc::Rc};
///
///let prod1 = prod!(
///    "x",
///    "y",
///    "z",
///    Exit {
///        var: "z".to_string()
///    },
///    ["z"]
///);
///let prod2 = Op {
///    fst: "x".to_string(),
///    op: BinOp::Prod,
///    snd: "y".to_string(),
///    var: "z".to_string(),
///    next: Rc::new(Statement::from(Exit {
///        var: "z".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["z".to_string()])),
///};
///assert_eq!(prod1, prod2)
///```
#[proc_macro]
pub fn prod(input: TokenStream) -> TokenStream {
    statements::prod(input)
}

///Create a [`axcut::syntax::statements::op::Op`] with
///[`axcut::syntax::statements::op::BinOp::Rem`]. If `free_vars_next` is not
///provided, it defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    exit::Exit,
///    op::{BinOp, Op},
///    Statement,
///};
///use axcut_macros::rem;
///use std::{collections::HashSet, rc::Rc};
///
///let rem1 = rem!(
///    "x",
///    "y",
///    "z",
///    Exit {
///        var: "z".to_string()
///    },
///    ["z"]
///);
///let rem2 = Op {
///    fst: "x".to_string(),
///    op: BinOp::Rem,
///    snd: "y".to_string(),
///    var: "z".to_string(),
///    next: Rc::new(Statement::from(Exit {
///        var: "z".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["z".to_string()])),
///};
///assert_eq!(rem1, rem2)
///```
#[proc_macro]
pub fn rem(input: TokenStream) -> TokenStream {
    statements::rem(input)
}

///Create a [`axcut::syntax::statements::op::Op`] with
///[`axcut::syntax::statements::op::BinOp::Sum`]. If `free_vars_next` is not
///provided, it defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    exit::Exit,
///    op::{BinOp, Op},
///    Statement,
///};
///use axcut_macros::sum;
///use std::{collections::HashSet, rc::Rc};
///
///let sum1 = sum!(
///    "x",
///    "y",
///    "z",
///    Exit {
///        var: "z".to_string()
///    },
///    ["z"]
///);
///let sum2 = Op {
///    fst: "x".to_string(),
///    op: BinOp::Sum,
///    snd: "y".to_string(),
///    var: "z".to_string(),
///    next: Rc::new(Statement::from(Exit {
///        var: "z".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["z".to_string()])),
///};
///assert_eq!(sum1, sum2)
///```
#[proc_macro]
pub fn sum(input: TokenStream) -> TokenStream {
    statements::sum(input)
}

///Create a [`axcut::syntax::statements::op::Op`] with
///[`axcut::syntax::statements::op::BinOp::Sub`]. If `free_vars_next` is not
///provided, it defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    exit::Exit,
///    op::{BinOp, Op},
///    Statement,
///};
///use axcut_macros::sub;
///use std::{collections::HashSet, rc::Rc};
///
///let sub1 = sub!(
///    "x",
///    "y",
///    "z",
///    Exit {
///        var: "z".to_string()
///    },
///    ["z"]
///);
///let sub2 = Op {
///    fst: "x".to_string(),
///    op: BinOp::Sub,
///    snd: "y".to_string(),
///    var: "z".to_string(),
///    next: Rc::new(Statement::from(Exit {
///        var: "z".to_string(),
///    })),
///    free_vars_next: Some(HashSet::from(["z".to_string()])),
///};
///assert_eq!(sub1, sub2)
///```
#[proc_macro]
pub fn sub(input: TokenStream) -> TokenStream {
    statements::sub(input)
}

///Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
///[`axcut::syntax::statements::ifc::IfSort::Equal`]. `snd` is optional and
///defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    ifc::{IfC, IfSort},
///    Exit, Statement,
///};
///use axcut_macros::ife;
///use std::rc::Rc;
///
///let if1 = ife!(
///    "x",
///    "y",
///    Exit {
///        var: "x".to_string()
///    },
///    Exit {
///        var: "y".to_string()
///    }
///);
///let if2 = IfC {
///    sort: IfSort::Equal,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    elsec: Rc::new(Statement::from(Exit {
///        var: "y".to_string(),
///    })),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ife(input: TokenStream) -> TokenStream {
    statements::ife(input)
}

///Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
///[`axcut::syntax::statements::ifc::IfSort::NotEqual`]. `snd` is optional and
///defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    ifc::{IfC, IfSort},
///    Exit, Statement,
///};
///use axcut_macros::ifne;
///use std::rc::Rc;
///
///let if1 = ifne!(
///    "x",
///    "y",
///    Exit {
///        var: "x".to_string()
///    },
///    Exit {
///        var: "y".to_string()
///    }
///);
///let if2 = IfC {
///    sort: IfSort::NotEqual,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    elsec: Rc::new(Statement::from(Exit {
///        var: "y".to_string(),
///    })),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifne(input: TokenStream) -> TokenStream {
    statements::ifne(input)
}

///Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
///[`axcut::syntax::statements::ifc::IfSort::Less`]. `snd` is optional and defaults
///to `None`
///
///```
///use axcut::syntax::statements::{
///    ifc::{IfC, IfSort},
///    Exit, Statement,
///};
///use axcut_macros::ifl;
///use std::rc::Rc;
///
///let if1 = ifl!(
///    "x",
///    "y",
///    Exit {
///        var: "x".to_string()
///    },
///    Exit {
///        var: "y".to_string()
///    }
///);
///let if2 = IfC {
///    sort: IfSort::Less,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    elsec: Rc::new(Statement::from(Exit {
///        var: "y".to_string(),
///    })),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifl(input: TokenStream) -> TokenStream {
    statements::ifl(input)
}

///Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
///[`axcut::syntax::statements::ifc::IfSort::LessOrEqual`]. `snd` is optional and
///defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    ifc::{IfC, IfSort},
///    Exit, Statement,
///};
///use axcut_macros::ifle;
///use std::rc::Rc;
///
///let if1 = ifle!(
///    "x",
///    "y",
///    Exit {
///        var: "x".to_string()
///    },
///    Exit {
///        var: "y".to_string()
///    }
///);
///let if2 = IfC {
///    sort: IfSort::LessOrEqual,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    elsec: Rc::new(Statement::from(Exit {
///        var: "y".to_string(),
///    })),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifle(input: TokenStream) -> TokenStream {
    statements::ifle(input)
}

///Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
///[`axcut::syntax::statements::ifc::IfSort::Greater`]. `snd` is optional and
///defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    ifc::{IfC, IfSort},
///    Exit, Statement,
///};
///use axcut_macros::ifg;
///use std::rc::Rc;
///
///let if1 = ifg!(
///    "x",
///    "y",
///    Exit {
///        var: "x".to_string()
///    },
///    Exit {
///        var: "y".to_string()
///    }
///);
///let if2 = IfC {
///    sort: IfSort::Greater,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    elsec: Rc::new(Statement::from(Exit {
///        var: "y".to_string(),
///    })),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifg(input: TokenStream) -> TokenStream {
    statements::ifg(input)
}

///Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
///[`axcut::syntax::statements::ifc::IfSort::GreaterOrEqual`]. `snd` is optional
///and defaults to `None`
///
///```
///use axcut::syntax::statements::{
///    ifc::{IfC, IfSort},
///    Exit, Statement,
///};
///use axcut_macros::ifge;
///use std::rc::Rc;
///
///let if1 = ifge!(
///    "x",
///    "y",
///    Exit {
///        var: "x".to_string()
///    },
///    Exit {
///        var: "y".to_string()
///    }
///);
///let if2 = IfC {
///    sort: IfSort::GreaterOrEqual,
///    fst: "x".to_string(),
///    snd: Some("y".to_string()),
///    thenc: Rc::new(Statement::from(Exit {
///        var: "x".to_string(),
///    })),
///    elsec: Rc::new(Statement::from(Exit {
///        var: "y".to_string(),
///    })),
///};
///assert_eq!(if1, if2);
///```
#[proc_macro]
pub fn ifge(input: TokenStream) -> TokenStream {
    statements::ifge(input)
}

// Declarations
///Create a [`axcut::syntax::declaration::XtorSig`]
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    declaration::XtorSig,
///    types::Ty,
///};
///use axcut_macros::xtor_sig;
///let xtor1 = xtor_sig!(
///    "Cons",
///    [
///        ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Ext,
///            ty: Ty::I64
///        },
///        ContextBinding {
///            var: "xs".to_string(),
///            chi: Chirality::Prd,
///            ty: Ty::Decl("ListInt".to_string())
///        }
///    ]
///);
///let xtor2 = XtorSig {
///    name: "Cons".to_string(),
///    args: TypingContext {
///        bindings: vec![
///            ContextBinding {
///                var: "x".to_string(),
///                chi: Chirality::Ext,
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
///assert_eq!(xtor1, xtor2)
///```
#[proc_macro]
pub fn xtor_sig(input: TokenStream) -> TokenStream {
    declarations::xtor_sig(input)
}

///Create a [`axcut::syntax::def::Def`]. `used_vars` is optional and defaults to
///`HashSet::new()`
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    def::Def,
///    statements::{Exit, Statement},
///    types::Ty,
///};
///use axcut_macros::def;
///use std::collections::HashSet;
///
///let def1 = def!(
///    "exit",
///    [ContextBinding {
///        var: "x".to_string(),
///        chi: Chirality::Ext,
///        ty: Ty::I64
///    }],
///    Exit {
///        var: "x".to_string()
///    },
///    ["x"]
///);
///let def2 = Def {
///    name: "exit".to_string(),
///    context: TypingContext {
///        bindings: vec![ContextBinding {
///            var: "x".to_string(),
///            chi: Chirality::Ext,
///            ty: Ty::I64,
///        }],
///    },
///    body: Statement::from(Exit {
///        var: "x".to_string(),
///    }),
///    used_vars: HashSet::from(["x".to_string()]),
///};
///assert_eq!(def1, def2);
///```
#[proc_macro]
pub fn def(input: TokenStream) -> TokenStream {
    declarations::def(input)
}

///Create a [`axcut::syntax::declaration::TypeDeclaration`]
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    declaration::{TypeDeclaration, XtorSig},
///    types::Ty,
///};
///use axcut_macros::ty_decl;
///let decl1 = ty_decl!(
///    "ListInt",
///    [
///        XtorSig {
///            name: "Nil".to_string(),
///            args: TypingContext { bindings: vec![] }
///        },
///        XtorSig {
///            name: "Cons".to_string(),
///            args: TypingContext {
///                bindings: vec![
///                    ContextBinding {
///                        var: "x".to_string(),
///                        chi: Chirality::Ext,
///                        ty: Ty::I64
///                    },
///                    ContextBinding {
///                        var: "xs".to_string(),
///                        chi: Chirality::Prd,
///                        ty: Ty::Decl("ListInt".to_string())
///                    }
///                ]
///            }
///        }
///    ]
///);
///let decl2 = TypeDeclaration {
///    name: "ListInt".to_string(),
///    xtors: vec![
///        XtorSig {
///            name: "Nil".to_string(),
///            args: TypingContext { bindings: vec![] },
///        },
///        XtorSig {
///            name: "Cons".to_string(),
///            args: TypingContext {
///                bindings: vec![
///                    ContextBinding {
///                        var: "x".to_string(),
///                        chi: Chirality::Ext,
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
///assert_eq!(decl1, decl2)
///```
#[proc_macro]
pub fn ty_decl(input: TokenStream) -> TokenStream {
    declarations::ty_decl(input)
}

///Create a [`axcut::syntax::program::Prog`]
///
///```
///use axcut::syntax::{
///    context::{Chirality, ContextBinding, TypingContext},
///    declaration::{TypeDeclaration, XtorSig},
///    def::Def,
///    program::Prog,
///    statements::{Exit, Statement},
///    types::Ty,
///};
///use axcut_macros::prog;
///use std::collections::HashSet;
///
///let prog1 = prog!(
///    [Def {
///        name: "main".to_string(),
///        context: TypingContext { bindings: vec![] },
///        body: Statement::from(Exit {
///            var: "x".to_string()
///        }),
///        used_vars: HashSet::from(["x".to_string()])
///    }],
///    [TypeDeclaration {
///        name: "ListInt".to_string(),
///        xtors: vec![
///            XtorSig {
///                name: "Nil".to_string(),
///                args: TypingContext { bindings: vec![] }
///            },
///            XtorSig {
///                name: "Cons".to_string(),
///                args: TypingContext {
///                    bindings: vec![
///                        ContextBinding {
///                            var: "x".to_string(),
///                            chi: Chirality::Ext,
///                            ty: Ty::I64
///                        },
///                        ContextBinding {
///                            var: "xs".to_string(),
///                            chi: Chirality::Prd,
///                            ty: Ty::Decl("ListInt".to_string())
///                        }
///                    ]
///                }
///            }
///        ]
///    }]
///);
///let prog2 = Prog {
///    defs: vec![Def {
///        name: "main".to_string(),
///        context: TypingContext { bindings: vec![] },
///        body: Statement::from(Exit {
///            var: "x".to_string(),
///        }),
///        used_vars: HashSet::from(["x".to_string()]),
///    }],
///    types: vec![TypeDeclaration {
///        name: "ListInt".to_string(),
///        xtors: vec![
///            XtorSig {
///                name: "Nil".to_string(),
///                args: TypingContext { bindings: vec![] },
///            },
///            XtorSig {
///                name: "Cons".to_string(),
///                args: TypingContext {
///                    bindings: vec![
///                        ContextBinding {
///                            var: "x".to_string(),
///                            chi: Chirality::Ext,
///                            ty: Ty::I64,
///                        },
///                        ContextBinding {
///                            var: "xs".to_string(),
///                            chi: Chirality::Prd,
///                            ty: Ty::Decl("ListInt".to_string()),
///                        },
///                    ],
///                },
///            },
///        ],
///    }],
///};
///assert_eq!(prog1, prog2)
///```
#[proc_macro]
pub fn prog(input: TokenStream) -> TokenStream {
    program::prog(input)
}
