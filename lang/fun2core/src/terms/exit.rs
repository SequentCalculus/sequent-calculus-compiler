//! This module defines the translation for the exit term.

use crate::{
    compile::{Compile, CompilePoly, CompileState},
    types::{compile_ty, compile_ty_poly},
};
use core_lang::syntax::{Ty, terms::Cns};

use std::rc::Rc;

impl Compile for fun::syntax::terms::Exit {
    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚exit t 〛_{c} = exit 〚t〛
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile_with_cont(
        self,
        _: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statements::Exit {
            arg: Rc::new(self.arg.compile(state, Ty::I64)),
            ty: compile_ty(
                &self
                    .ty
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into()
    }
}

impl CompilePoly for fun::syntax::terms::Exit {
    fn compile_with_cont_poly(
        self,
        _: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
        type_params: &std::collections::HashMap<String, core_lang::syntax::names::Identifier>,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statements::Exit {
            arg: Rc::new(self.arg.compile_poly(state, Ty::I64, type_params)),
            ty: compile_ty_poly(
                &self
                    .ty
                    .expect("Types should be annotated before translation"),
                type_params,
            ),
        }
        .into()
    }
}
