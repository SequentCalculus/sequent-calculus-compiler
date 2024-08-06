use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::App {
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let new_cont = core::syntax::Destructor {
            id: core::syntax::Dtor::Ap,
            producers: vec![Rc::unwrap_or_clone(self.argument).compile_opt(st)],
            consumers: vec![cont],
        }
        .into();
        Rc::unwrap_or_clone(self.function).compile_with_cont(new_cont, st)
    }
}
