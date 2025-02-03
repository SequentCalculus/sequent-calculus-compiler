use crate::definition::{CompileState, CompileWithCont};
use core_lang::syntax::{term::Cns, Ty};
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::PrintLnI64 {
    /// ```text
    /// 〚println_i64(t_1); t_2 〛_{c} = println_i64(〚t_1〛); 〚t_2 〛_{c}
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statement::PrintLnI64 {
            arg: Rc::new(self.arg.compile_opt(state, Ty::I64)),
            next: Rc::new(self.next.compile_with_cont(cont.clone(), state)),
        }
        .into()
    }
}
