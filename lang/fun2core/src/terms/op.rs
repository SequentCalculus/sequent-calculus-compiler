use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Op {
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::Op {
            fst: Rc::new(self.fst.compile_opt(st)),
            op: self.op.compile(st),
            snd: Rc::new(self.snd.compile_opt(st)),
            continuation: Rc::new(cont),
        }
        .into()
    }
}
