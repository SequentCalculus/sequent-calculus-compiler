//! This module defines the translation of parenthesized terms.

use crate::compile::{Compile, CompileState};
use core_lang::syntax::{
    Ty,
    terms::{Cns, Prd},
};

impl Compile for fun::syntax::terms::Paren {
    fn compile(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        self.inner.compile(state, ty)
    }

    fn compile_with_cont(
        self,
        c: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        self.inner.compile_with_cont(c, state)
    }
}
