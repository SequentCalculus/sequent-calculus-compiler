use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::Op {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p1 = self.fst.compile(state);
        let p2 = self.snd.compile(state);
        state.add_covars(Rc::as_ref(&p1));
        state.add_covars(Rc::as_ref(&p2));
        let new_cv = state.free_covar_from_state();
        let new_op = core::syntax::Op {
            fst: p1,
            op: self.op.compile(state),
            snd: p2,
            continuation: Rc::new(core::syntax::Consumer::Covar(new_cv.clone())),
        }
        .into();
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_op),
        }
        .into()
    }
}

impl CompileWithCont for fun::syntax::Op {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Op;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st.into()),
        }
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        core::syntax::Op {
            fst: self.fst.compile_opt(st),
            op: self.op.compile(st),
            snd: self.snd.compile_opt(st),
            continuation: Rc::new(cont),
        }
    }
}
