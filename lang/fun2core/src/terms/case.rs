//! Compile [fun::syntax::terms::Case]
use crate::{
    compile::{share, CompileState, CompileWithCont},
    terms::clause::compile_clause,
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;
use fun::syntax::types::OptTyped;

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Case {
    /// ```text
    /// 〚case t of { K_1(x_11, ...) => t_1, ...} 〛_{c} = 〚t〛_{case{ K_1(x_11, ...) => 〚t_1〛_{μ~x.share(fv(c), x)}, ... }}
    /// WITH
    /// def share(fv(c), x) { < x | c > }
    /// ```
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
                    .destructee
                    .get_type()
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into();

        // 〚t〛_{new_cont}
        Rc::unwrap_or_clone(self.destructee).compile_with_cont(new_cont, state)
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{CompileState, CompileWithCont};
    use core_lang::syntax::terms::{Cns, Prd};
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_list,
        typing::check::Check,
    };

    use std::collections::{HashSet, VecDeque};
    use std::rc::Rc;

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
        let result = term_typed.compile_opt(&mut state, core_lang::syntax::types::Ty::I64);

        let mut context = core_lang::syntax::TypingContext::default();
        context.add_var("x", core_lang::syntax::types::Ty::I64);
        context.add_var(
            "xs",
            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
        );
        let mut subst = core_lang::syntax::substitution::Substitution::default();
        subst.add_prod(core_lang::syntax::terms::Literal::new(1));
        subst.add_prod(core_lang::syntax::terms::Xtor::ctor(
            "Nil",
            core_lang::syntax::substitution::Substitution::default(),
            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
        ));
        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::Cut::new(
                core_lang::syntax::terms::Xtor {
                    prdcns: Prd,
                    id: "Cons".to_owned(),
                    args: subst,
                    ty: core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
                },
                core_lang::syntax::terms::XCase {
                    prdcns: Cns,
                    clauses: vec![
                        core_lang::syntax::terms::Clause {
                            prdcns: Cns,
                            xtor: "Nil".to_owned(),
                            context: core_lang::syntax::TypingContext::default(),
                            body: Rc::new(
                                core_lang::syntax::statements::Cut::new(
                                    core_lang::syntax::terms::Literal::new(0),
                                    core_lang::syntax::terms::XVar::covar(
                                        "a0",
                                        core_lang::syntax::types::Ty::I64,
                                    ),
                                    core_lang::syntax::types::Ty::I64,
                                )
                                .into(),
                            ),
                        },
                        core_lang::syntax::terms::Clause {
                            prdcns: Cns,
                            xtor: "Cons".to_owned(),
                            context,
                            body: Rc::new(
                                core_lang::syntax::statements::Cut::new(
                                    core_lang::syntax::terms::XVar::var(
                                        "x",
                                        core_lang::syntax::types::Ty::I64,
                                    ),
                                    core_lang::syntax::terms::XVar::covar(
                                        "a0",
                                        core_lang::syntax::types::Ty::I64,
                                    ),
                                    core_lang::syntax::types::Ty::I64,
                                )
                                .into(),
                            ),
                        },
                    ],
                    ty: core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
                },
                core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected);
    }
}
