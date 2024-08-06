use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Label {
    type TargetInner = core::syntax::Cut;
    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st.into()),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_cont = core::syntax::Consumer::Covar(self.label.clone());
        let new_st = self.term.compile_with_cont(new_cont, st);
        let new_mu = core::syntax::Mu {
            covariable: self.label,
            statement: new_st,
        };
        core::syntax::Cut {
            producer: Rc::new(new_mu.into()),
            consumer: Rc::new(cont),
        }
    }
}
