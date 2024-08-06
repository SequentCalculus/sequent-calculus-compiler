use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl Compile for fun::syntax::IfZ {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p1 = self.ifc.compile(state);
        let p2 = self.thenc.compile(state);
        let p3 = self.elsec.compile(state);
        state.add_covars(&p1);
        state.add_covars(&p2);
        state.add_covars(&p3);
        let new_cv = state.free_covar_from_state();
        let new_cons = Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
        let s1 = Rc::new(
            core::syntax::Cut {
                producer: p2,
                consumer: new_cons.clone(),
            }
            .into(),
        );
        let s2 = Rc::new(
            core::syntax::Cut {
                producer: p3,
                consumer: new_cons,
            }
            .into(),
        );
        let new_if = Rc::new(
            core::syntax::IfZ {
                ifc: p1,
                thenc: s1,
                elsec: s2,
            }
            .into(),
        );
        core::syntax::Mu {
            covariable: new_cv,
            statement: new_if,
        }
        .into()
    }
}

impl CompileWithCont for fun::syntax::IfZ {
    type Target = core::syntax::Mu;
    type TargetInner = core::syntax::IfZ;

    fn compile_opt(self, st: &mut CompileState) -> Self::Target {
        let new_cv = st.free_covar_from_state();
        let new_cont = core::syntax::Consumer::Covar(new_cv.clone());
        let new_st = self.compile_with_cont(new_cont, st);
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
        core::syntax::IfZ {
            ifc: self.ifc.compile_opt(st),
            thenc: self.thenc.compile_with_cont(cont.clone(), st),
            elsec: self.elsec.compile_with_cont(cont, st),
        }
    }
}
