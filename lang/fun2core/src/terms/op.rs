use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Op {
    type TargetInner = core::syntax::Op;

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
        core::syntax::Op {
            fst: Rc::new(self.fst.compile_opt(st)),
            op: self.op.compile(st),
            snd: Rc::new(self.snd.compile_opt(st)),
            continuation: Rc::new(cont),
        }
    }
}
