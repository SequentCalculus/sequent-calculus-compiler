use std::rc::Rc;

use crate::definition::CompileWithCont;
use core_lang::syntax::{
    term::{Cns, Prd},
    types::Ty,
};

impl CompileWithCont for fun::syntax::terms::Lit {
    fn compile_opt(
        self,
        _state: &mut crate::definition::CompileState,
        _ty: Ty,
    ) -> core_lang::syntax::term::Term<Prd> {
        core_lang::syntax::term::Literal { lit: self.val }.into()
    }

    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        _state: &mut crate::definition::CompileState,
    ) -> core_lang::syntax::Statement {
        let new_lit: core_lang::syntax::term::Term<Prd> =
            core_lang::syntax::term::Literal { lit: self.val }.into();
        core_lang::syntax::statement::Cut {
            producer: Rc::new(new_lit),
            ty: Ty::I64,
            consumer: Rc::new(cont),
        }
        .into()
    }
}
