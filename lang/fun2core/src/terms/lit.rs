//! This module defines the translation of integer literals.

use crate::compile::{Compile, CompilePoly};
use core_lang::syntax::{
    Identifier, Ty,
    terms::{Cns, Prd},
};

use std::{collections::HashMap, rc::Rc};

impl Compile for fun::syntax::terms::Lit {
    /// This implementation of [Compile::compile] proceeds as follows.
    /// ```text
    /// 〚n 〛 = n
    /// ```
    fn compile(
        self,
        _state: &mut crate::compile::CompileState,
        _ty: Ty,
    ) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::Literal { lit: self.lit }.into()
    }

    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚n 〛_{c} = ⟨n | c⟩
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        _state: &mut crate::compile::CompileState,
    ) -> core_lang::syntax::Statement {
        let new_lit: core_lang::syntax::terms::Term<Prd> =
            core_lang::syntax::terms::Literal { lit: self.lit }.into();
        core_lang::syntax::statements::Cut {
            producer: Rc::new(new_lit),
            ty: Ty::I64,
            consumer: Rc::new(cont),
        }
        .into()
    }
}

impl CompilePoly for fun::syntax::terms::Lit {
    fn compile_poly(
        self,
        _state: &mut crate::compile::CompileState,
        _ty: Ty,
        _typed_args: &HashMap<String, Identifier>,
    ) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::Literal { lit: self.lit }.into()
    }

    fn compile_with_cont_poly(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        _state: &mut crate::compile::CompileState,
        _type_args: &HashMap<String, Identifier>,
    ) -> core_lang::syntax::Statement {
        let new_lit: core_lang::syntax::terms::Term<Prd> =
            core_lang::syntax::terms::Literal { lit: self.lit }.into();
        core_lang::syntax::statements::Cut {
            producer: Rc::new(new_lit),
            ty: Ty::I64,
            consumer: Rc::new(cont),
        }
        .into()
    }
}
