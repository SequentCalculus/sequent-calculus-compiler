use crate::compile::{CompileState, CompileWithCont};
use core_lang::syntax::Ty;

pub mod call;
pub mod case;
pub mod cocase;
pub mod constructor;
pub mod destructor;
pub mod goto;
pub mod ifc;
pub mod ifz;
pub mod label;
pub mod r#let;
pub mod lit;
pub mod op;
pub mod paren;
pub mod print;
pub mod variable;

impl CompileWithCont for fun::syntax::terms::Term {
    fn compile_opt(
        self,
        state: &mut CompileState,
        ty: Ty,
    ) -> core_lang::syntax::terms::Term<core_lang::syntax::terms::Prd> {
        match self {
            fun::syntax::terms::Term::XVar(var) => var.compile_opt(state, ty),
            fun::syntax::terms::Term::Lit(lit) => lit.compile_opt(state, ty),
            fun::syntax::terms::Term::Op(op) => op.compile_opt(state, ty),
            fun::syntax::terms::Term::IfC(ifc) => ifc.compile_opt(state, ty),
            fun::syntax::terms::Term::IfZ(ifz) => ifz.compile_opt(state, ty),
            fun::syntax::terms::Term::PrintLnI64(print) => print.compile_opt(state, ty),
            fun::syntax::terms::Term::Let(r#let) => r#let.compile_opt(state, ty),
            fun::syntax::terms::Term::Call(call) => call.compile_opt(state, ty),
            fun::syntax::terms::Term::Constructor(ctor) => ctor.compile_opt(state, ty),
            fun::syntax::terms::Term::Destructor(dtor) => dtor.compile_opt(state, ty),
            fun::syntax::terms::Term::Case(case) => case.compile_opt(state, ty),
            fun::syntax::terms::Term::Cocase(cocase) => cocase.compile_opt(state, ty),
            fun::syntax::terms::Term::Goto(goto) => goto.compile_opt(state, ty),
            fun::syntax::terms::Term::Label(label) => label.compile_opt(state, ty),
            fun::syntax::terms::Term::Paren(paren) => paren.compile_opt(state, ty),
        }
    }

    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<core_lang::syntax::terms::Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        match self {
            fun::syntax::terms::Term::XVar(var) => var.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Lit(lit) => lit.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Op(op) => op.compile_with_cont(cont, state),
            fun::syntax::terms::Term::IfC(ifc) => ifc.compile_with_cont(cont, state),
            fun::syntax::terms::Term::IfZ(ifz) => ifz.compile_with_cont(cont, state),
            fun::syntax::terms::Term::PrintLnI64(print) => print.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Let(r#let) => r#let.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Call(call) => call.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Constructor(ctor) => ctor.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Destructor(dtor) => dtor.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Case(case) => case.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Cocase(cocase) => cocase.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Goto(goto) => goto.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Label(label) => label.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Paren(paren) => paren.compile_with_cont(cont, state),
        }
    }
}
