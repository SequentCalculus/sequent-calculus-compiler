use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::IfZ {
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::IfZ {
            ifc: Rc::new(self.ifc.compile_opt(st)),
            thenc: Rc::new(self.thenc.compile_with_cont(cont.clone(), st)),
            elsec: Rc::new(self.elsec.compile_with_cont(cont, st)),
        }
        .into()
    }
}
