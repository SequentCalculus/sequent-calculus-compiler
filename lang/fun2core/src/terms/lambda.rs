use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Lam {
    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        let new_cv = st.free_covar_from_state();
        core::syntax::Cocase {
            cocases: vec![core::syntax::Clause {
                xtor: core::syntax::Dtor::Ap,
                vars: vec![self.variable],
                covars: vec![new_cv.clone()],
                rhs: Rc::new(
                    self.body
                        .compile_with_cont(core::syntax::Consumer::Covar(new_cv), st),
                ),
            }],
        }
        .into()
    }
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::Cut {
            producer: Rc::new(self.compile_opt(st)),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
