use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Let {
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let new_cont = core::syntax::MuTilde {
            variable: self.variable,
            statement: Rc::new(self.bound_term.compile_with_cont(cont, st)),
        };
        self.in_term.compile_with_cont(new_cont.into(), st)
    }
}
