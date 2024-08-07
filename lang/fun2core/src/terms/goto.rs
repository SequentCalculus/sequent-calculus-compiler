use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Goto {
    /// ```text
    /// 〚goto(t;a) 〛_{c} = 〚t〛_{a}
    /// ```
    fn compile_with_cont(
        self,
        _: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        self.term
            .compile_with_cont(core::syntax::Consumer::Covar(self.target), st)
    }
}
