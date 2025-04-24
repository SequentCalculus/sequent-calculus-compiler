use crate::compile::{CompileState, CompileWithCont};
use core_lang::syntax::{Ty, terms::Cns};

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::PrintI64 {
    /// ```text
    /// 〚println_i64(t_1); t_2 〛_{c} = println_i64(〚t_1〛); 〚t_2 〛_{c}
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statements::PrintI64 {
            newline: self.newline,
            arg: Rc::new(self.arg.compile_opt(state, Ty::I64)),
            next: Rc::new(self.next.compile_with_cont(cont.clone(), state)),
        }
        .into()
    }
}
