//! This module defines the translation of a copattern match.

use crate::{
    compile::{Compile, CompileState},
    terms::clause::compile_coclause,
    types::compile_ty,
};
use core_lang::syntax::{
    Ty,
    terms::{Cns, Prd},
};

use std::rc::Rc;

impl Compile for fun::syntax::terms::New {
    /// This implementation of [Compile::compile] proceeds as follows.
    /// ```text
    /// 〚new { D_1(x_11, ...) => t_1, ...} 〛 = cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... }
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile(self, state: &mut CompileState, _ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::XCase {
            prdcns: Prd,
            clauses: self
                .clauses
                .into_iter()
                .map(|clause| compile_coclause(clause, state))
                .collect(),
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
    /// 〚new { D_1(x_11, ...) => t_1, ...} 〛_{c} =
    ///   ⟨cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... } | c⟩
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
    use core_macros::{bind, clause, cns, cocase, codata, covar, cut, lit, ty};
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_lpair,
        typing::check::Check,
    };
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn compile_lpair() {
        let term = parse_term!("new { fst => 1, snd => 2 }");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &TypingContext::default(),
                &fun::syntax::types::Ty::mk_decl(
                    "LPair",
                    fun::syntax::types::TypeArgs::mk(vec![
                        fun::syntax::types::Ty::mk_i64(),
                        fun::syntax::types::Ty::mk_i64(),
                    ]),
                ),
            )
            .unwrap();

        let lpair_declaration = codata!("LPair", []);

        let mut state = CompileState {
            used_vars: HashSet::default(),
            codata_types: &[lpair_declaration],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("LPair[i64, i64]"));

        let expected = cocase!(
            [
                clause!(
                    core_syntax::Prd,
                    "fst",
                    [bind!("a0", cns!()),],
                    cut!(lit!(1), covar!("a0"))
                ),
                clause!(
                    core_syntax::Prd,
                    "snd",
                    [bind!("a1", cns!())],
                    cut!(lit!(2), covar!("a1"))
                )
            ],
            ty!("LPair[i64, i64]")
        )
        .into();

        assert_eq!(result, expected);
    }
}
