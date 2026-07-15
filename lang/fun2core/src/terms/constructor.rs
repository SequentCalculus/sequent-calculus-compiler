//! This module defines the translation of a constructor.

use crate::{
    arguments::compile_subst,
    compile::{Compile, CompileState},
    types::{compile_ty, compile_type_args},
};
use core_lang::syntax::{
    Ty,
    names::Identifier,
    terms::{Cns, Prd},
};

use std::{collections::HashMap, rc::Rc};

impl Compile for fun::syntax::terms::Constructor {
    /// This implementation of [Compile::compile] proceeds as follows.
    /// ```text
    /// 〚K(t_1, ...) 〛 = K( 〚t_1〛, ...)
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile(
        self,
        state: &mut CompileState,
        _ty: Ty,
        type_params: Rc<HashMap<String, Identifier>>,
    ) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::Xtor {
            prdcns: Prd,
            name: Identifier::new(self.id),
            type_args: compile_type_args(&self.type_args, type_params.clone()),
            args: compile_subst(self.args, state, type_params.clone()),
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
    /// 〚K(t_1, ...) 〛_{c} = ⟨K( 〚t_1〛, ...) | c⟩
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
        type_params: Rc<HashMap<String, Identifier>>,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(
            &self
                .ty
                .clone()
                .expect("Types should be annotated before translation"),
            type_params.clone(),
        );
        core_lang::syntax::statements::Cut {
            producer: Rc::new(self.compile(state, ty.clone(), type_params)),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use core_macros::{ctor, id, lit, ty};
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_list,
        typing::check::Check,
    };
    use std::{
        collections::{HashSet, VecDeque},
        rc::Rc,
    };

    #[test]
    fn compile_cons() {
        let term = parse_term!("Cons(1,Nil)");
        let term_typed = term
            .check(
                &mut symbol_table_list(),
                &TypingContext::default(),
                &fun::syntax::types::Ty::mk_decl(
                    "List",
                    fun::syntax::types::TypeArgs::mk(vec![fun::syntax::types::Ty::mk_i64()]),
                ),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::default(),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };
        let result = term_typed.compile(
            &mut state,
            ty!(id!("List"), vec![ty!("int")]),
            Rc::default(),
        );

        let expected = ctor!(
            id!("Cons"),
            [],
            [
                lit!(1),
                ctor!(id!("Nil"), [], [], ty!(id!("List"), vec![ty!("int")]))
            ],
            ty!(id!("List"), vec![ty!("int")])
        )
        .into();
        assert_eq!(result, expected)
    }
}
