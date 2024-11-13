use std::rc::Rc;

use crate::{definition::CompileWithCont, program::compile_ty};
use core::syntax::{
    term::{Cns, Prd},
    types::Ty,
};

impl CompileWithCont for fun::syntax::terms::Var {
    fn compile_opt(
        self,
        _state: &mut crate::definition::CompileState,
        _ty: Ty,
    ) -> core::syntax::term::Term<Prd> {
        core::syntax::term::XVar {
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
        cont: core::syntax::term::Term<Cns>,
        _state: &mut crate::definition::CompileState,
    ) -> core::syntax::Statement {
        let ty = compile_ty(
            self.ty
                .expect("Types should be annotated before translation"),
        );
        let new_var: core::syntax::term::Term<Prd> = core::syntax::term::XVar {
            prdcns: Prd,
            var: self.var,
            ty: ty.clone(),
        }
        .into();
        core::syntax::statement::Cut {
            producer: Rc::new(new_var),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}
