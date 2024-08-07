use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Destructor {
    /// ```text
    /// 〚t.D(t_1,...) 〛_{c} =  〚t〛_{D(〚t_1〛,...);c)}
    ///
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        // new continuation: D(〚t_1〛,...);c)
        let new_cont = core::syntax::Destructor {
            id: self.id.compile(st),
            producers: self.args.into_iter().map(|p| p.compile_opt(st)).collect(),
            consumers: vec![cont],
        }
        .into();
        // 〚t〛_{new_cont}
        self.destructee.compile_with_cont(new_cont, st)
    }
}
