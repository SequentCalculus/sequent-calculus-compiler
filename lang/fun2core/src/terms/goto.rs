use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::Goto {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        state.covars.insert(self.target.clone());
        let p = self.term.compile(state);
        state.add_covars(Rc::as_ref(&p));
        let new_cv = state.free_covar_from_state();
        let new_covar: Rc<core::syntax::Consumer> =
            Rc::new(core::syntax::Consumer::Covar(self.target));
        let new_cut: Rc<core::syntax::Statement> = Rc::new(
            core::syntax::Cut {
                producer: p,
                consumer: new_covar,
            }
            .into(),
        );
        core::syntax::Mu {
            covariable: new_cv,
            statement: new_cut,
        }
        .into()
    }
}

impl CompileWithCont for fun::syntax::Goto {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Statement;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
    }

    fn compile_with_cont(
        self,
        _: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        Rc::unwrap_or_clone(self.term)
            .compile_with_cont(core::syntax::Consumer::Covar(self.target), st)
    }
}
