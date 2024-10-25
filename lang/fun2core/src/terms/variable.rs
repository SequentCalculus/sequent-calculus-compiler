use std::rc::Rc;

use crate::definition::CompileWithCont;
use core::syntax::{
    term::{Cns, Prd},
    types::Ty,
};

impl CompileWithCont for fun::syntax::terms::Var {
    fn compile_opt(
        self,
        _state: &mut crate::definition::CompileState,
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
        _state: &mut crate::definition::CompileState,
    ) -> core::syntax::Statement {
        let new_var: core::syntax::term::Term<Prd> = core::syntax::term::XVar {
            prdcns: Prd,
            var: self.var,
        }
        .into();
        core::syntax::statement::Cut {
            producer: Rc::new(new_var),
            //TODO get correct type
            ty: Ty::Int(),
            consumer: Rc::new(cont),
        }
        .into()
    }
}
