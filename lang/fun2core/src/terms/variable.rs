use std::rc::Rc;

use crate::definition::CompileWithCont;

impl CompileWithCont for fun::syntax::terms::Var {
    fn compile_opt(self, _state: &mut crate::definition::CompileState) -> core::syntax::Producer {
        core::syntax::Variable { var: self.var }.into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        _state: &mut crate::definition::CompileState,
    ) -> core::syntax::Statement {
        let new_var: core::syntax::Producer = core::syntax::Variable { var: self.var }.into();
        core::syntax::statement::Cut {
            producer: Rc::new(new_var),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
