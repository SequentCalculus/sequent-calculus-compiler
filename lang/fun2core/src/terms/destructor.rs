//! This module defines the translation of a destructor.

use crate::{
    arguments::compile_subst,
    compile::{Compile, CompileState},
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;
use fun::syntax::types::OptTyped;

impl Compile for fun::syntax::terms::Destructor {
    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚t.D(t_1, ...) 〛_{c} = 〚t〛_{D(〚t_1〛, ..., c)}
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
        let mut args = compile_subst(self.args, state);
        args.entries.push(cont.into());
        // new continuation: D(〚t_1〛, ..., c)
        let new_cont = core_lang::syntax::terms::Xtor {
            prdcns: Cns,
            id: self.id,
            args,
            ty: compile_ty(
                &self
                    .scrutinee
                    .get_type()
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into();

        // 〚t〛_{new_cont}
        self.scrutinee.compile_with_cont(new_cont, state)
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use core_lang::syntax::terms::Prd;
    use core_macros::{bind, clause, cns, cocase, covar, cut, dtor, lit, mu, ty};
    use fun::{parse_term, test_common::symbol_table_lpair, typing::check::Check};
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn compile_fst() {
        let term = parse_term!("new { fst => 1, snd => 2}.fst[i64, i64]");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::default(),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, core_lang::syntax::types::Ty::I64);

        let expected = mu!(
            ("a", 0),
            cut!(
                cocase!(
                    [
                        clause!(
                            Prd,
                            "fst",
                            [bind!("a", 1, cns!())],
                            cut!(lit!(1), covar!("a", 1))
                        ),
                        clause!(
                            Prd,
                            "snd",
                            [bind!("a", 2, cns!())],
                            cut!(lit!(2), covar!("a", 2))
                        )
                    ],
                    ty!("LPair[i64, i64]")
                ),
                dtor!("fst", [covar!("a", 0)], ty!("LPair[i64, i64]")),
                ty!("LPair[i64, i64]")
            )
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_snd() {
        let term = parse_term!("new { fst => 1, snd => 2}.snd[i64, i64]");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::default(),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("int"));

        let expected = mu!(
            ("a", 0),
            cut!(
                cocase!(
                    [
                        clause!(
                            Prd,
                            "fst",
                            [bind!("a", 1, cns!())],
                            cut!(lit!(1), covar!("a", 1))
                        ),
                        clause!(
                            Prd,
                            "snd",
                            [bind!("a", 2, cns!())],
                            cut!(lit!(2), covar!("a", 2))
                        )
                    ],
                    ty!("LPair[i64, i64]")
                ),
                dtor!("snd", [covar!("a", 0)], ty!("LPair[i64, i64]")),
                ty!("LPair[i64, i64]")
            )
        )
        .into();
        assert_eq!(result, expected)
    }
}
