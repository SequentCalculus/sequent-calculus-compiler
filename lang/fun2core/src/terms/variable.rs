//! This module defines the translation for variables.

use crate::{compile::Compile, types::compile_ty};
use core_lang::syntax::{
    Ty,
    names::Identifier,
    terms::{Cns, Prd},
    type_params::ParamPolarity,
};

use std::{collections::HashMap, rc::Rc};

impl Compile for fun::syntax::terms::XVar {
    /// This implementation of [Compile::compile] proceeds as follows.
    /// ```text
    /// 〚v 〛 = v
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile(
        self,
        _state: &mut crate::compile::CompileState,
        _ty: Ty,
        type_params: Rc<HashMap<String, (Identifier, ParamPolarity)>>,
    ) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::XVar {
            prdcns: Prd,
            var: Identifier::new(self.var),
            ty: compile_ty(
                &self
                    .ty
                    .expect("Types should be annotated before translation"),
                type_params,
            ),
        }
        .into()
    }

    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚v 〛_{c} = ⟨v | c⟩
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        _state: &mut crate::compile::CompileState,
        type_params: Rc<HashMap<String, (Identifier, ParamPolarity)>>,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(
            &self
                .ty
                .expect("Types should be annotated before translation"),
            type_params,
        );
        let new_var: core_lang::syntax::terms::Term<Prd> = core_lang::syntax::terms::XVar {
            prdcns: Prd,
            var: Identifier::new(self.var),
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
