use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::Destructor {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p = self.destructee.compile(state);
        state.add_covars(Rc::as_ref(&p));
        let args_comp: Vec<core::syntax::Producer> = self
            .args
            .iter()
            .cloned()
            .map(|arg| arg.compile(state))
            .collect();
        state.add_covars(&args_comp);
        let new_cv = state.free_covar_from_state();
        let new_dt: Rc<core::syntax::Consumer> = Rc::new(
            core::syntax::Destructor {
                id: self.id.compile(state),
                producers: args_comp,
                consumers: vec![core::syntax::Consumer::Covar(new_cv.clone())],
            }
            .into(),
        );
        let new_cut: Rc<core::syntax::Statement> = Rc::new(
            core::syntax::Cut {
                producer: p,
                consumer: new_dt,
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

impl CompileWithCont for fun::syntax::Destructor {
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
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_cont = core::syntax::Destructor {
            id: self.id.compile(st),
            producers: self
                .args
                .iter()
                .cloned()
                .map(|p| p.compile_opt(st))
                .collect(),
            consumers: vec![cont],
        }
        .into();
        Rc::unwrap_or_clone(self.destructee).compile_with_cont(new_cont, st)
    }
}
