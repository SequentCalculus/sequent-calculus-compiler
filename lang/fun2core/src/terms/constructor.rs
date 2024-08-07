use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Constructor {
    /// ```text
    /// 〚K(t_1,...) 〛_{c} =⟨ K( 〚t_1〛,...) | c⟩
    /// 〚K(t_1,...) 〛 = K( 〚t_1〛,...)
    ///
    /// ```
    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        let new_prods = self
            .args
            .iter()
            .cloned()
            .map(|t| t.compile_opt(st))
            .collect();
        core::syntax::Constructor {
            id: self.id.compile(st),
            producers: new_prods,
            consumers: vec![],
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::Cut {
            producer: Rc::new(self.compile_opt(st)),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
