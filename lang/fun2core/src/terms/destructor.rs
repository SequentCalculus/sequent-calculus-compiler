use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Destructor {
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let new_cont = core::syntax::Destructor {
            id: self.id.compile(st),
            producers: self.args.into_iter().map(|p| p.compile_opt(st)).collect(),
            consumers: vec![cont],
        }
        .into();
        self.destructee.compile_with_cont(new_cont, st)
    }
}
