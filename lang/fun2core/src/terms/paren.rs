use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Paren {
    type Target = core::syntax::Producer;

    type TargetInner = core::syntax::Statement;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        (*self.inner.compile_opt(st)).clone()
    }

    fn compile_with_cont(
        self,
        c: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        (*self.inner.compile_with_cont(c, st)).clone()
    }
}
