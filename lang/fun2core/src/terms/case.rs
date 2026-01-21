//! This module defines the translation of a pattern match.

use crate::{
    compile::{Compile, CompileState, share},
    terms::clause::compile_clause,
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;
use fun::syntax::types::OptTyped;

use std::rc::Rc;

impl Compile for fun::syntax::terms::Case {
    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚case t of { K_1(x_11, ...) => t_1, ...} 〛_{c} =
    ///   〚t〛_{case{ K_1(x_11, ...) => 〚t_1〛_{μ~x.share(fv(c), x)}, ... }}
    /// WITH
    /// def share(fv(c), x) { < x | c > }
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
        // if there is more than one clause and the consumer is a not a leaf, we share it by
        // lifting it to the top level to avoid exponential blowup
        let cont = if self.clauses.len() <= 1
            || matches!(
                cont,
                core_lang::syntax::Term::XVar(_)
            )
            // check if consumer is μ~x.exit p with p a leaf
            || matches!(&cont, core_lang::syntax::Term::Mu(core_lang::syntax::terms::Mu { statement, .. })
                if (matches!(&**statement, core_lang::syntax::Statement::Exit(core_lang::syntax::statements::Exit { arg, .. })
                    if matches!(**arg, core_lang::syntax::Term::XVar(_)) || matches!(**arg, core_lang::syntax::Term::Literal(_))))
            ) {
            cont
        } else {
            share(cont, state)
        };

        // new continuation: case{ K_1(x_11,...) => 〚t_1〛_{cont}, ... }
        let new_cont = core_lang::syntax::terms::XCase {
            prdcns: Cns,
            clauses: self
                .clauses
                .into_iter()
                .map(|clause| compile_clause(clause, cont.clone(), state))
                .collect(),
            ty: compile_ty(
                &self
                    .scrutinee
                    .get_type()
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into();

        // 〚t〛_{new_cont}
        Rc::unwrap_or_clone(self.scrutinee).compile_with_cont(new_cont, state)
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
    use macros::{bind, case, clause, covar, ctor, cut, mu, ty, var};

    use std::collections::{HashSet, VecDeque};

    #[test]
    fn compile_list() {
        let term = parse_term!("(Cons(1,Nil)).case[i64] { Nil => 0, Cons(x,xs) => x }");
        let term_typed = term
            .check(
                &mut symbol_table_list(),
                &TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string(), "xs".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("int"));

        let expected = mu!(
            "a0",
            cut!(
                ctor!(
                    "Cons",
                    [
                        core_syntax::Literal::new(1),
                        ctor!("Nil", [], ty!("List[i64]"))
                    ],
                    ty!("List[i64]")
                ),
                case!(
                    [
                        clause!(
                            core_syntax::Cns,
                            "Nil",
                            [],
                            cut!(core_syntax::Literal::new(0), covar!("a0"))
                        ),
                        clause!(
                            core_syntax::Cns,
                            "Cons",
                            [
                                bind!("x", core_syntax::Chirality::Prd),
                                bind!("xs", core_syntax::Chirality::Prd, ty!("List[i64]"))
                            ],
                            cut!(var!("x"), covar!("a0"))
                        )
                    ],
                    ty!("List[i64]")
                ),
                ty!("List[i64]")
            )
        )
        .into();
        assert_eq!(result, expected);
    }
}
