use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Label {
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let new_cont = core::syntax::Consumer::Covar(self.label.clone());
        let new_st = self.term.compile_with_cont(new_cont, st);
        let new_mu = core::syntax::Mu {
            covariable: self.label,
            statement: Rc::new(new_st),
        };
        core::syntax::Cut {
            producer: Rc::new(new_mu.into()),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
