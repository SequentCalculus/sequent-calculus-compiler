use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Fun {
    type TargetInner = core::syntax::Fun;

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
        let mut new_coargs: Vec<core::syntax::Consumer> = self
            .coargs
            .iter()
            .cloned()
            .map(core::syntax::Consumer::Covar)
            .collect();
        new_coargs.push(cont);
        let new_args = self
            .args
            .iter()
            .cloned()
            .map(|p| p.compile_opt(st))
            .collect();
        core::syntax::Fun {
            name: self.name,
            producers: new_args,
            consumers: new_coargs,
        }
    }
}
