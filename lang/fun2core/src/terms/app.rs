use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::App {
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
