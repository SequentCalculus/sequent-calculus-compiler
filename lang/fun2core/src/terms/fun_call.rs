use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::Fun {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let mut args_comp: Vec<core::syntax::Producer> = vec![];
        for arg in self.args.iter().cloned() {
            let arg_comp: core::syntax::Producer = arg.compile(state);
            state.add_covars(&arg_comp);
            args_comp.insert(0, arg_comp);
        }
        for cv in self.coargs.iter() {
            state.covars.insert(cv.clone());
        }
        let new_cv = state.free_covar_from_state();
        let new_covar: core::syntax::Consumer = core::syntax::Consumer::Covar(new_cv.clone());
        let mut new_cvs: Vec<core::syntax::Consumer> = self
            .coargs
            .iter()
            .map(|cv| core::syntax::Consumer::Covar(cv.clone()))
            .collect();
        new_cvs.insert(new_cvs.len(), new_covar);
        let new_fun: Rc<core::syntax::Statement> = Rc::new(
            core::syntax::Fun {
                name: self.name,
                producers: args_comp,
                consumers: new_cvs,
            }
            .into(),
        );
        core::syntax::Mu {
            covariable: new_cv,
            statement: new_fun,
        }
        .into()
    }
}

impl CompileWithCont for fun::syntax::Fun {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::Fun;

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
