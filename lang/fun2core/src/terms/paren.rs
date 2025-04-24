use crate::compile::{CompileState, CompileWithCont};
use core_lang::syntax::{
    Ty,
    terms::{Cns, Prd},
};

impl CompileWithCont for fun::syntax::terms::Paren {
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        self.inner.compile_opt(state, ty)
    }

    fn compile_with_cont(
        self,
        c: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        self.inner.compile_with_cont(c, state)
    }
}
