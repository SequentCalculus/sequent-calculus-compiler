use std::rc::Rc;

use crate::definition::CompileWithCont;

impl CompileWithCont for fun::syntax::terms::Lit {
    fn compile_opt(self, _state: &mut crate::definition::CompileState) -> core::syntax::Producer {
        core::syntax::Literal { lit: self.val }.into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        _state: &mut crate::definition::CompileState,
    ) -> core::syntax::Statement {
        let new_lit: core::syntax::Producer = core::syntax::Literal { lit: self.val }.into();
        core::syntax::statement::Cut {
            producer: Rc::new(new_lit),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
