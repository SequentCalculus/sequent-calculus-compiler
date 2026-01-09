//! This module defines the translation into [Core](core_lang) for each term the surface language
//! [Fun](fun).

use crate::compile::{Compile, CompileState};
use core_lang::syntax::Ty;

pub mod call;
pub mod call_template;
pub mod case;
pub mod clause;
pub mod constructor;
pub mod destructor;
pub mod exit;
pub mod goto;
pub mod ifc;
pub mod label;
pub mod r#let;
pub mod lit;
pub mod new;
pub mod op;
pub mod paren;
pub mod print;
pub mod variable;

impl Compile for fun::syntax::terms::Term {
    fn compile(
        self,
        state: &mut CompileState,
        ty: Ty,
    ) -> core_lang::syntax::terms::Term<core_lang::syntax::terms::Prd> {
        match self {
            fun::syntax::terms::Term::XVar(var) => var.compile(state, ty),
            fun::syntax::terms::Term::Lit(lit) => lit.compile(state, ty),
            fun::syntax::terms::Term::Op(op) => op.compile(state, ty),
            fun::syntax::terms::Term::IfC(ifc) => ifc.compile(state, ty),
            fun::syntax::terms::Term::PrintI64(print) => print.compile(state, ty),
            fun::syntax::terms::Term::Let(r#let) => r#let.compile(state, ty),
            fun::syntax::terms::Term::Call(call) => call.compile(state, ty),
            fun::syntax::terms::Term::CallTemplate(ct) => ct.compile(state, ty),
            fun::syntax::terms::Term::Constructor(ctor) => ctor.compile(state, ty),
            fun::syntax::terms::Term::Destructor(dtor) => dtor.compile(state, ty),
            fun::syntax::terms::Term::Case(case) => case.compile(state, ty),
            fun::syntax::terms::Term::New(new) => new.compile(state, ty),
            fun::syntax::terms::Term::Goto(goto) => goto.compile(state, ty),
            fun::syntax::terms::Term::Label(label) => label.compile(state, ty),
            fun::syntax::terms::Term::Exit(exit) => exit.compile(state, ty),
            fun::syntax::terms::Term::Paren(paren) => paren.compile(state, ty),
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
            fun::syntax::terms::Term::PrintI64(print) => print.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Let(r#let) => r#let.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Call(call) => call.compile_with_cont(cont, state),
            fun::syntax::terms::Term::CallTemplate(ct) => ct.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Constructor(ctor) => ctor.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Destructor(dtor) => dtor.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Case(case) => case.compile_with_cont(cont, state),
            fun::syntax::terms::Term::New(new) => new.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Goto(goto) => goto.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Label(label) => label.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Exit(exit) => exit.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Paren(paren) => paren.compile_with_cont(cont, state),
        }
    }
}
