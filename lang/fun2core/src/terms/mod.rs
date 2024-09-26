use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

pub mod case;
pub mod cocase;
pub mod constructor;
pub mod destructor;
pub mod fun_call;
pub mod goto;
pub mod idents;
pub mod ifz;
pub mod label;
pub mod let_exp;
pub mod op;
pub mod paren;

impl CompileWithCont for fun::syntax::terms::Term {
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        match self {
            fun::syntax::terms::Term::Var(v) => core::syntax::Variable { var: v.var }.into(),
            fun::syntax::terms::Term::Lit(n) => core::syntax::Literal { lit: n.val }.into(),
            fun::syntax::terms::Term::Op(op) => op.compile_opt(state),
            fun::syntax::terms::Term::IfZ(ifz) => ifz.compile_opt(state),
            fun::syntax::terms::Term::Let(lt) => lt.compile_opt(state),
            fun::syntax::terms::Term::Fun(fun) => fun.compile_opt(state),
            fun::syntax::terms::Term::Constructor(cons) => cons.compile_opt(state),
            fun::syntax::terms::Term::Destructor(dest) => dest.compile_opt(state),
            fun::syntax::terms::Term::Case(case) => case.compile_opt(state),
            fun::syntax::terms::Term::Cocase(cocase) => cocase.compile_opt(state),
            fun::syntax::terms::Term::Goto(goto) => goto.compile_opt(state),
            fun::syntax::terms::Term::Label(label) => label.compile_opt(state),
            fun::syntax::terms::Term::Paren(paren) => paren.compile_opt(state),
        }
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        match self {
            fun::syntax::terms::Term::Var(v) => {
                let new_var: core::syntax::Producer = core::syntax::Variable { var: v.var }.into();
                core::syntax::Cut {
                    producer: Rc::new(new_var),
                    consumer: Rc::new(cont),
                }
                .into()
            }
            fun::syntax::terms::Term::Lit(n) => {
                let new_lit: core::syntax::Producer = core::syntax::Literal { lit: n.val }.into();
                core::syntax::Cut {
                    producer: Rc::new(new_lit),
                    consumer: Rc::new(cont),
                }
                .into()
            }
            fun::syntax::terms::Term::Op(op) => op.compile_with_cont(cont, state),
            fun::syntax::terms::Term::IfZ(ifz) => ifz.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Let(lt) => lt.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Fun(fun) => fun.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Constructor(cons) => cons.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Destructor(dest) => dest.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Case(case) => case.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Cocase(cocase) => cocase.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Goto(goto) => goto.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Label(label) => label.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Paren(paren) => paren.compile_with_cont(cont, state),
        }
    }
}
