use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Goto {
    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        _: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        Rc::unwrap_or_clone(self.term)
            .compile_with_cont(core::syntax::Consumer::Covar(self.target), st)
    }
}
