use crate::definition::{CompileState, CompileWithCont};
use core::syntax::{
    term::{Cns, Prd},
    types::Ty,
};

impl CompileWithCont for fun::syntax::terms::Paren {
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core::syntax::term::Term<Prd> {
        self.inner.compile_opt(state, ty)
    }

    fn compile_with_cont(
        self,
        c: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        self.inner.compile_with_cont(c, state)
    }
}
