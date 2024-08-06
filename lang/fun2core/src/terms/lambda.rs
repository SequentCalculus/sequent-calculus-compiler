use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::Lam {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p = self.body.compile(state);
        state.add_covars(Rc::as_ref(&p));
        let new_cv = state.free_covar_from_state();
        let new_covar: Rc<core::syntax::Consumer> =
            Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
        let new_rhs: Rc<core::syntax::Statement> = Rc::new(
            core::syntax::Cut {
                producer: p,
                consumer: new_covar,
            }
            .into(),
        );
        let new_pt: core::syntax::Clause<core::syntax::Dtor> = core::syntax::Clause {
            xtor: core::syntax::Dtor::Ap,
            vars: vec![self.variable],
            covars: vec![new_cv],
            rhs: new_rhs,
        };
        core::syntax::Cocase {
            cocases: vec![new_pt],
        }
        .into()
    }
}

impl CompileWithCont for fun::syntax::Lam {
    type Target = core::syntax::Cocase;
    type TargetInner = core::syntax::Cut;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
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
    }
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner {
        let new_prod = self.compile_opt(st).into();
        core::syntax::Cut {
            producer: Rc::new(new_prod),
            consumer: Rc::new(cont),
        }
    }
}
