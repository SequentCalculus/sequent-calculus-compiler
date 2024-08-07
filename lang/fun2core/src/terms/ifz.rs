use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::IfZ {
    /// ```text
    /// 〚IfZ(t1,t2,t3) 〛_{c} = IfZ(〚t_1 〛 〚t_2 〛_{c}, 〚t_3 〛_{c})
    ///
    /// ```
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
