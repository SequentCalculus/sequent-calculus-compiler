use std::rc::Rc;

use crate::{definition::CompileWithCont, program::compile_ty};
use core_lang::syntax::{
    term::{Cns, Prd},
    types::Ty,
};

impl CompileWithCont for fun::syntax::terms::Var {
    fn compile_opt(
        self,
        _state: &mut crate::definition::CompileState,
        _ty: Ty,
    ) -> core_lang::syntax::term::Term<Prd> {
        core_lang::syntax::term::XVar {
            prdcns: Prd,
            var: self.var,
            ty: compile_ty(
                self.ty
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        _state: &mut crate::definition::CompileState,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(
            self.ty
                .expect("Types should be annotated before translation"),
        );
        let new_var: core_lang::syntax::term::Term<Prd> = core_lang::syntax::term::XVar {
            prdcns: Prd,
            var: self.var,
            ty: ty.clone(),
        }
        .into();
        core_lang::syntax::statement::Cut {
            producer: Rc::new(new_var),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}
