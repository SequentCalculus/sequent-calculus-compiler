use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::App {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p1 = self.function.compile(state);
        state.add_covars(Rc::as_ref(&p1));
        let p2 = self.argument.compile(state);
        state.add_covars(&p2);
        let new_cv = state.free_covar_from_state();
        let new_covar: core::syntax::Consumer = core::syntax::Consumer::Covar(new_cv.clone());
        let new_dt: Rc<core::syntax::Consumer> = Rc::new(
            core::syntax::Destructor {
                id: core::syntax::Dtor::Ap,
                producers: vec![(*p2).clone()],
                consumers: vec![new_covar],
            }
            .into(),
        );
        let new_cut: Rc<core::syntax::Statement> = Rc::new(
            core::syntax::Cut {
                producer: p1,
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

impl CompileWithCont for fun::syntax::App {
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
            id: core::syntax::Dtor::Ap,
            producers: vec![Rc::unwrap_or_clone(self.argument).compile_opt(st)],
            consumers: vec![cont],
        }
        .into();
        Rc::unwrap_or_clone(self.function).compile_with_cont(new_cont, st)
    }
}
