use crate::definition::{CompileState, CompileWithCont};
use std::rc::Rc;

impl CompileWithCont for fun::syntax::Let {
    /// ```text
    /// 〚let x:=t_1 in t_2 〛_{c} = 〚t_1 〛_{μ~x.〚t_2 〛_c }
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        // new continuation : μ~x.〚t_2 〛_c
        let new_cont = core::syntax::MuTilde {
            variable: self.variable,
            statement: Rc::new(self.bound_term.compile_with_cont(cont, st)),
        };
        self.in_term.compile_with_cont(new_cont.into(), st)
    }
}
