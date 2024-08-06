use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Goto {
    fn compile_with_cont(
        self,
        _: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        self.term
            .compile_with_cont(core::syntax::Consumer::Covar(self.target), st)
    }
}
