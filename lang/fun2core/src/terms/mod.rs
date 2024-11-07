use crate::definition::{CompileState, CompileWithCont};
use core::syntax::types::Ty;

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
pub mod lit;
pub mod op;
pub mod paren;
pub mod variable;

impl CompileWithCont for fun::syntax::terms::Term {
    fn compile_opt(
        self,
        state: &mut CompileState,
        ty: Ty,
    ) -> core::syntax::term::Term<core::syntax::term::Prd> {
        match self {
            fun::syntax::terms::Term::Var(v) => v.compile_opt(state, ty),
            fun::syntax::terms::Term::Lit(n) => n.compile_opt(state, ty),
            fun::syntax::terms::Term::Op(op) => op.compile_opt(state, ty),
            fun::syntax::terms::Term::IfZ(ifz) => ifz.compile_opt(state, ty),
            fun::syntax::terms::Term::Let(lt) => lt.compile_opt(state, ty),
            fun::syntax::terms::Term::Fun(fun) => fun.compile_opt(state, ty),
            fun::syntax::terms::Term::Constructor(cons) => cons.compile_opt(state, ty),
            fun::syntax::terms::Term::Destructor(dest) => dest.compile_opt(state, ty),
            fun::syntax::terms::Term::Case(case) => case.compile_opt(state, ty),
            fun::syntax::terms::Term::Cocase(cocase) => cocase.compile_opt(state, ty),
            fun::syntax::terms::Term::Goto(goto) => goto.compile_opt(state, ty),
            fun::syntax::terms::Term::Label(label) => label.compile_opt(state, ty),
            fun::syntax::terms::Term::Paren(paren) => paren.compile_opt(state, ty),
        }
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<core::syntax::term::Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        match self {
            fun::syntax::terms::Term::Var(v) => v.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Lit(n) => n.compile_with_cont(cont, state),
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
