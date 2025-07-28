//! Compilation for [fun::syntax::terms::Lit]
use crate::compile::CompileWithCont;
use core_lang::syntax::{
    Ty,
    terms::{Cns, Prd},
};

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Lit {
    /// ```text
    /// 〚n 〛 = n
    /// ```
    fn compile_opt(
        self,
        _state: &mut crate::compile::CompileState,
        _ty: Ty,
    ) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::Literal { lit: self.val }.into()
    }

    /// ```text
    /// 〚n 〛_{c} = ⟨n | c⟩
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        _state: &mut crate::compile::CompileState,
    ) -> core_lang::syntax::Statement {
        let new_lit: core_lang::syntax::terms::Term<Prd> =
            core_lang::syntax::terms::Literal { lit: self.val }.into();
        core_lang::syntax::statements::Cut {
            producer: Rc::new(new_lit),
            ty: Ty::I64,
            consumer: Rc::new(cont),
        }
        .into()
    }
}
