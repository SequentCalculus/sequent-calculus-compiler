use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Cocase {
    type Target = core::syntax::Cocase;
    type TargetInner = core::syntax::Cut;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        core::syntax::Cocase {
            cocases: self
                .cocases
                .iter()
                .cloned()
                .map(|cc| cc.compile_opt(st))
                .collect(),
        }
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_cocase = self.compile_opt(st).into();
        core::syntax::Cut {
            producer: Rc::new(new_cocase),
            consumer: Rc::new(cont),
        }
    }
}

impl CompileWithCont for fun::syntax::Clause<fun::syntax::Dtor> {
    type Target = core::syntax::Clause<core::syntax::Dtor>;
    type TargetInner = core::syntax::Clause<core::syntax::Dtor>;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        core::syntax::Clause {
            xtor: self.xtor.compile(st),
            vars: self.vars,
            covars: vec![new_cv.clone()],
            rhs: Rc::new(
                self.rhs
                    .compile_with_cont(core::syntax::Consumer::Covar(new_cv), st),
            ),
        }
    }
    fn compile_with_cont(
        self,
        _: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        self.compile_opt(st)
    }
}
