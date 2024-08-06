use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Destructor {
    type TargetInner = core::syntax::Statement;

    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(core::syntax::Consumer::Covar(new_cv.clone()), st);
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
        .into()
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
