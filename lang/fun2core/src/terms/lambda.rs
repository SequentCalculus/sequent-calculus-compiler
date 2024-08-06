use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Lam {
    type TargetInner = core::syntax::Cut;

    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        let new_cv = st.free_covar_from_state();
        core::syntax::Cocase {
            cocases: vec![core::syntax::Clause {
                xtor: core::syntax::Dtor::Ap,
                vars: vec![self.variable],
                covars: vec![new_cv.clone()],
                rhs: self
                    .body
                    .compile_with_cont(core::syntax::Consumer::Covar(new_cv), st),
            }],
        }
        .into()
    }
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_prod = self.compile_opt(st);
        core::syntax::Cut {
            producer: Rc::new(new_prod),
            consumer: Rc::new(cont),
        }
    }
}
