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
                .map(|clause| compile_clause(clause, st))
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

fn compile_clause(
    clause: fun::syntax::Clause<fun::syntax::Dtor>,
    st: &mut CompileState,
) -> core::syntax::Clause<core::syntax::Dtor> {
    let new_cv = st.free_covar_from_state();
    core::syntax::Clause {
        xtor: clause.xtor.compile(st),
        vars: clause.vars,
        covars: vec![new_cv.clone()],
        rhs: Rc::new(
            clause
                .rhs
                .compile_with_cont(core::syntax::Consumer::Covar(new_cv), st),
        ),
    }
}
