//! This module defines the translation for printing an integer.

use crate::compile::{Compile, CompilePoly, CompileState};
use core_lang::syntax::{Identifier, Ty, terms::Cns};

use std::{collections::HashMap, rc::Rc};

impl Compile for fun::syntax::terms::PrintI64 {
    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
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
            arg: Rc::new(self.arg.compile(state, Ty::I64)),
            next: Rc::new(self.next.compile_with_cont(cont.clone(), state)),
        }
        .into()
    }
}

impl CompilePoly for fun::syntax::terms::PrintI64 {
    fn compile_with_cont_poly(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
        type_params: Rc<HashMap<String, Identifier>>,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statements::PrintI64 {
            newline: self.newline,
            arg: Rc::new(self.arg.compile_poly(state, Ty::I64, type_params.clone())),
            next: Rc::new(
                self.next
                    .compile_with_cont_poly(cont.clone(), state, type_params),
            ),
        }
        .into()
    }
}
