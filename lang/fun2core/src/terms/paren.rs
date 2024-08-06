use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Paren {
    type TargetInner = core::syntax::Statement;

    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        self.inner.compile_opt(st)
    }

    fn compile_with_cont(
        self,
        c: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        (*self.inner.compile_with_cont(c, st)).clone()
    }
}
