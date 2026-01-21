//! This module defines the translation of a constructor.

use crate::{
    arguments::compile_subst,
    compile::{Compile, CompileState},
    types::compile_ty,
};
use core_lang::syntax::{
    Ty,
    terms::{Cns, Prd},
};

use std::rc::Rc;

impl Compile for fun::syntax::terms::Constructor {
    /// This implementation of [Compile::compile] proceeds as follows.
    /// ```text
    /// 〚K(t_1, ...) 〛 = K( 〚t_1〛, ...)
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile(self, state: &mut CompileState, _ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::Xtor {
            prdcns: Prd,
            id: self.id,
            args: compile_subst(self.args, state),
            ty: compile_ty(
                &self
                    .ty
                    .expect("Types should be annotated before translation"),
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
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(
            &self
                .ty
                .clone()
                .expect("Types should be annotated before translation"),
        );
        core_lang::syntax::statements::Cut {
            producer: Rc::new(self.compile(state, ty.clone())),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use core_lang::syntax as core_syntax;
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_list,
        typing::check::Check,
    };
    use macros::{ctor, ty};
    use std::collections::{HashSet, VecDeque};

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
        };
        let result = term_typed.compile(&mut state, ty!("List[i64]"));

        let expected = ctor!(
            "Cons",
            [
                core_syntax::Literal::new(1),
                ctor!("Nil", [], ty!("List[i64]"))
            ],
            ty!("List[i64]")
        )
        .into();
        assert_eq!(result, expected)
    }
}
