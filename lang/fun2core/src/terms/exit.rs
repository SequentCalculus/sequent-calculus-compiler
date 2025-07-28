//! Compilation for [fun::syntax::terms::Exit]
use crate::{
    compile::{CompileState, CompileWithCont},
    types::compile_ty,
};
use core_lang::syntax::{terms::Cns, Ty};

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Exit {
    /// ```text
    /// 〚exit t 〛_{c} = exit 〚t〛
    /// ```
    fn compile_with_cont(
        self,
        _: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statements::Exit {
            arg: Rc::new(self.arg.compile_opt(state, Ty::I64)),
            ty: compile_ty(
                &self
                    .ty
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into()
    }
}
