use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::IfZ {
    type TargetInner = core::syntax::IfZ;

    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        let new_cv = st.free_covar_from_state();
        let new_cont = core::syntax::Consumer::Covar(new_cv.clone());
        let new_st = self.compile_with_cont(new_cont, st);
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
        core::syntax::IfZ {
            ifc: Rc::new(self.ifc.compile_opt(st)),
            thenc: self.thenc.compile_with_cont(cont.clone(), st),
            elsec: self.elsec.compile_with_cont(cont, st),
        }
    }
}
