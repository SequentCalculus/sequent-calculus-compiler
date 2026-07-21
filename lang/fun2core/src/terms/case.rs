//! This module defines the translation of a pattern match.

use crate::{
    compile::{Compile, CompileState, share},
    terms::clause::compile_clause,
    types::compile_ty,
};
use core_lang::syntax::{Identifier, terms::Cns};
use fun::traits::OptTyped;

use std::{collections::HashMap, rc::Rc};

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
        type_params: Rc<HashMap<String, Identifier>>,
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
                .map(|clause| compile_clause(clause, cont.clone(), state, type_params.clone()))
                .collect(),
            ty: compile_ty(
                &self
                    .scrutinee
                    .get_type()
                    .expect("Types should be annotated before translation"),
                type_params.clone(),
            ),
        }
        .into();

        // 〚t〛_{new_cont}
        Rc::unwrap_or_clone(self.scrutinee).compile_with_cont(new_cont, state, type_params)
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use core_lang::syntax::{self as core_syntax};
    use core_macros::{
        bind, case, clause, covar, ctor, ctor_sig, cut, data, id, lit, mu, prd, tvar, ty, var,
    };
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_list,
        typing::check::Check,
    };

    use std::{
        collections::{HashSet, VecDeque},
        rc::Rc,
    };

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

        let list = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))]))
                    ]
                )
            ],
            [id!("A", 1)]
        );

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string(), "xs".to_string()]),
            codata_types: &[],
            data_types: &[list],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };
        let result = term_typed.compile(&mut state, ty!("int"), Rc::default());

        let expected = mu!(
            id!("a0"),
            cut!(
                ctor!(
                    id!("Cons", 0),
                    [],
                    [
                        lit!(1),
                        ctor!(id!("Nil", 0), [], [], ty!(id!("List"), vec![ty!("int")]))
                    ],
                    ty!(id!("List"), vec![ty!("int")])
                ),
                case!(
                    [
                        clause!(
                            core_syntax::Cns,
                            id!("Nil", 0),
                            [],
                            [],
                            cut!(lit!(0), covar!(id!("a0")))
                        ),
                        clause!(
                            core_syntax::Cns,
                            id!("Cons", 0),
                            [],
                            [
                                bind!(id!("x"), core_syntax::Chirality::Prd),
                                bind!(
                                    id!("xs"),
                                    core_syntax::Chirality::Prd,
                                    ty!(id!("List"), vec![ty!("int")])
                                )
                            ],
                            cut!(var!(id!("x")), covar!(id!("a0")))
                        )
                    ],
                    ty!(id!("List"), vec![ty!("int")])
                ),
                ty!(id!("List"), vec![ty!("int")])
            )
        )
        .into();
        assert_eq!(result, expected);
    }
}
