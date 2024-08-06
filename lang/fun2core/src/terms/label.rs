use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Label {
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let new_cont = core::syntax::Consumer::Covar(self.label.clone());

        core::syntax::Cut {
            producer: Rc::new(
                core::syntax::Mu {
                    covariable: self.label,
                    statement: Rc::new(self.term.compile_with_cont(new_cont, st)),
                }
                .into(),
            ),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
