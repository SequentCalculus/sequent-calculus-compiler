use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Constructor {
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
        let new_cons = self.compile_opt(st);
        core::syntax::Cut {
            producer: Rc::new(new_cons),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
