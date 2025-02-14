use crate::{compile::CompileWithCont, program::compile_ty};
use core_lang::syntax::{
    terms::{Cns, Prd},
    Ty,
};

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::XVar {
    /// ```text
    /// 〚v 〛 = v
    /// ```
    fn compile_opt(
        self,
        _state: &mut crate::compile::CompileState,
        _ty: Ty,
    ) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::XVar {
            prdcns: Prd,
            var: self.var,
            ty: compile_ty(
                &self
                    .ty
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into()
    }

    /// ```text
    /// 〚v 〛_{c} = ⟨v | c⟩
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        _state: &mut crate::compile::CompileState,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(
            &self
                .ty
                .expect("Types should be annotated before translation"),
        );
        let new_var: core_lang::syntax::terms::Term<Prd> = core_lang::syntax::terms::XVar {
            prdcns: Prd,
            var: self.var,
            ty: ty.clone(),
        }
        .into();
        core_lang::syntax::statements::Cut {
            producer: Rc::new(new_var),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}
