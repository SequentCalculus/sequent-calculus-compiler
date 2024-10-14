use std::rc::Rc;

use crate::definition::CompileWithCont;
use core::syntax::term::{Cns, Prd};

impl CompileWithCont for fun::syntax::terms::Lit {
    fn compile_opt(
        self,
        _state: &mut crate::definition::CompileState,
    ) -> core::syntax::term::Term<Prd> {
        core::syntax::term::Literal { lit: self.val }.into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        _state: &mut crate::definition::CompileState,
    ) -> core::syntax::Statement {
        let new_lit: core::syntax::term::Term<Prd> =
            core::syntax::term::Literal { lit: self.val }.into();
        core::syntax::statement::Cut {
            producer: Rc::new(new_lit),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
