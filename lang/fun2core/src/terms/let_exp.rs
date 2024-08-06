use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::Let {
    type Target = core::syntax::Producer;
    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p1 = self.bound_term.compile(state);
        let p2 = self.in_term.compile(state);
        state.add_covars(&p1);
        state.add_covars(&p2);
        let new_cv = state.free_covar_from_state();
        let new_cons = Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
        let cut_inner = Rc::new(
            core::syntax::Cut {
                producer: p2,
                consumer: new_cons,
            }
            .into(),
        );
        let new_mutilde = Rc::new(core::syntax::Consumer::MuTilde(core::syntax::MuTilde {
            variable: self.variable.clone(),
            statement: cut_inner,
        }));
        let cut_outer = Rc::new(
            core::syntax::Cut {
                producer: p1,
                consumer: new_mutilde,
            }
            .into(),
        );
        core::syntax::Mu {
            covariable: new_cv,
            statement: cut_outer,
        }
        .into()
    }
}

impl CompileWithCont for fun::syntax::Let {
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
        let new_st = self.bound_term.compile_with_cont(cont, st);
        let new_cont = core::syntax::MuTilde {
            variable: self.variable,
            statement: new_st,
        };
        Rc::unwrap_or_clone(self.in_term).compile_with_cont(new_cont.into(), st)
    }
}
