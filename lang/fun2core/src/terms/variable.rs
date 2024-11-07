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
        _: Ty,
    ) -> core::syntax::term::Term<Prd> {
        core::syntax::term::XVar {
            prdcns: Prd,
            var: self.var,
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        _: &mut crate::definition::CompileState,
    ) -> core::syntax::Statement {
        let new_var: core::syntax::term::Term<Prd> = core::syntax::term::XVar {
            prdcns: Prd,
            var: self.var,
        }
        .into();
        core::syntax::statement::Cut {
            producer: Rc::new(new_var),
            ty: compile_ty(self.ty.unwrap()),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
