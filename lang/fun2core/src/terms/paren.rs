//! This module defines the translation of parenthesized terms.

use std::{collections::HashMap, rc::Rc};

use crate::compile::{Compile, CompileState};
use core_lang::syntax::{
    Identifier, Ty,
    terms::{Cns, Prd},
};

impl Compile for fun::syntax::terms::Paren {
    fn compile(
        self,
        state: &mut CompileState,
        ty: Ty,
        type_params: Rc<HashMap<String, Identifier>>,
    ) -> core_lang::syntax::terms::Term<Prd> {
        self.inner.compile(state, ty, type_params)
    }

    fn compile_with_cont(
        self,
        c: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
        type_params: Rc<HashMap<String, Identifier>>,
    ) -> core_lang::syntax::Statement {
        self.inner.compile_with_cont(c, state, type_params)
    }
}
