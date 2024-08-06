use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

pub mod app;
pub mod case;
pub mod cocase;
pub mod constructor;
pub mod destructor;
pub mod fun_call;
pub mod goto;
pub mod idents;
pub mod ifz;
pub mod label;
pub mod lambda;
pub mod let_exp;
pub mod op;
pub mod paren;

impl CompileWithCont for fun::syntax::Term {
    type Target = core::syntax::Producer;
    type TargetInner = core::syntax::Statement;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Term::Var(v) => core::syntax::Variable { var: v }.into(),
            fun::syntax::Term::Lit(n) => core::syntax::Literal { lit: n }.into(),
            fun::syntax::Term::Op(op) => op.compile_opt(st).into(),
            fun::syntax::Term::IfZ(ifz) => ifz.compile_opt(st).into(),
            fun::syntax::Term::Let(lt) => lt.compile_opt(st).into(),
            fun::syntax::Term::Fun(fun) => fun.compile_opt(st).into(),
            fun::syntax::Term::Constructor(cons) => cons.compile_opt(st).into(),
            fun::syntax::Term::Destructor(dest) => dest.compile_opt(st).into(),
            fun::syntax::Term::Case(case) => case.compile_opt(st).into(),
            fun::syntax::Term::Cocase(cocase) => cocase.compile_opt(st).into(),
            fun::syntax::Term::Lam(lam) => lam.compile_opt(st).into(),
            fun::syntax::Term::App(ap) => ap.compile_opt(st).into(),
            fun::syntax::Term::Goto(goto) => goto.compile_opt(st).into(),
            fun::syntax::Term::Label(label) => label.compile_opt(st).into(),
            fun::syntax::Term::Paren(paren) => paren.compile_opt(st),
        }
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        match self {
            fun::syntax::Term::Var(v) => {
                let new_var: core::syntax::Producer = core::syntax::Variable { var: v }.into();
                core::syntax::Cut {
                    producer: Rc::new(new_var),
                    consumer: Rc::new(cont),
                }
                .into()
            }
            fun::syntax::Term::Lit(n) => {
                let new_lit: core::syntax::Producer = core::syntax::Literal { lit: n }.into();
                core::syntax::Cut {
                    producer: Rc::new(new_lit),
                    consumer: Rc::new(cont),
                }
                .into()
            }
            fun::syntax::Term::Op(op) => op.compile_with_cont(cont, st).into(),
            fun::syntax::Term::IfZ(ifz) => ifz.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Let(lt) => lt.compile_with_cont(cont, st),
            fun::syntax::Term::Fun(fun) => fun.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Constructor(cons) => cons.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Destructor(dest) => dest.compile_with_cont(cont, st),
            fun::syntax::Term::Case(case) => case.compile_with_cont(cont, st),
            fun::syntax::Term::Cocase(cocase) => cocase.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Lam(lam) => lam.compile_with_cont(cont, st).into(),
            fun::syntax::Term::App(ap) => ap.compile_with_cont(cont, st),
            fun::syntax::Term::Goto(goto) => goto.compile_with_cont(cont, st),
            fun::syntax::Term::Label(label) => label.compile_with_cont(cont, st).into(),
            fun::syntax::Term::Paren(paren) => paren.compile_with_cont(cont, st),
        }
    }
}
