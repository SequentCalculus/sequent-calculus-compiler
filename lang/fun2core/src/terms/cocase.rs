use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::Cocase {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let mut new_pts: Vec<core::syntax::Clause<core::syntax::Dtor>> = vec![];
        for pt in self.cocases.iter().cloned() {
            let pt_cloned: fun::syntax::Clause<fun::syntax::Dtor> = pt.clone();
            let rhs: Rc<core::syntax::Producer> = Rc::new(pt.rhs.compile(state));
            state.add_covars(Rc::as_ref(&rhs));
            let new_cv = state.free_covar_from_state();
            let new_covar = Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
            let new_cut: Rc<core::syntax::Statement> = Rc::new(
                core::syntax::Cut {
                    producer: rhs,
                    consumer: new_covar,
                }
                .into(),
            );
            let new_pt: core::syntax::Clause<core::syntax::Dtor> = core::syntax::Clause {
                xtor: pt_cloned.xtor.clone().compile(state),
                vars: pt_cloned.vars.clone(),
                covars: vec![new_cv],
                rhs: new_cut,
            };
            new_pts.insert(0, new_pt);
        }
        core::syntax::Cocase { cocases: new_pts }.into()
    }
}

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
