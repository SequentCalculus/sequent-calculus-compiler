use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_context, compile_ty},
};
use core_lang::syntax::{term::Cns, Statement};
use fun::syntax::types::OptTyped;

impl CompileWithCont for fun::syntax::terms::Case {
    /// ```text
    /// 〚case t of { K_1(x_11, ...) => t_1, ...} 〛_{c} = 〚t〛_{case{ K_1(x_11, ...) => 〚t_1〛_{c}, ... }}
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        // new continuation: case{ K_1(x_11,...) => 〚t_1〛_{c}, ... }
        let new_cont = core_lang::syntax::term::XCase {
            prdcns: Cns,
            clauses: self
                .cases
                .into_iter()
                .map(|clause| compile_clause(clause, cont.clone(), state))
                .collect(),
            ty: compile_ty(
                self.destructee
                    .get_type()
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into();

        // 〚t〛_{new_cont}
        Rc::unwrap_or_clone(self.destructee).compile_with_cont(new_cont, state)
    }
}

fn compile_clause(
    clause: fun::syntax::terms::Clause<fun::syntax::Name>,
    cont: core_lang::syntax::term::Term<Cns>,
    state: &mut CompileState,
) -> core_lang::syntax::term::Clause<Cns, Statement> {
    core_lang::syntax::term::Clause {
        prdcns: Cns,
        xtor: clause.xtor,
        context: compile_context(clause.context),
        rhs: Rc::new(clause.rhs.compile_with_cont(cont, state)),
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use core_lang::syntax::{
        context::Context,
        term::{Cns, Prd},
    };
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_list,
        typing::check::Check,
    };
    use std::rc::Rc;

    #[test]
    fn compile_list() {
        let term = parse_term!("(Cons(1,Nil)).case { Nil => 0, Cons(x : i64,xs : ListInt) => x }");
        let term_typed = term
            .check(
                &symbol_table_list(),
                &TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let mut ctx = Context::new();
        ctx.add_var("x", core_lang::syntax::types::Ty::I64);
        ctx.add_var(
            "xs",
            core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
        );
        let expected = core_lang::syntax::term::Mu::mu(
            "a0",
            core_lang::syntax::statement::Cut::new(
                core_lang::syntax::term::Xtor {
                    prdcns: Prd,
                    id: "Cons".to_owned(),
                    args: vec![
                        core_lang::syntax::substitution::SubstitutionBinding::ProducerBinding(
                            core_lang::syntax::term::Literal::new(1).into(),
                        ),
                        core_lang::syntax::substitution::SubstitutionBinding::ProducerBinding(
                            core_lang::syntax::term::Xtor {
                                prdcns: Prd,
                                id: "Nil".to_owned(),
                                args: vec![],
                                ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
                            }
                            .into(),
                        ),
                    ],
                    ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
                },
                core_lang::syntax::term::XCase {
                    prdcns: Cns,
                    clauses: vec![
                        core_lang::syntax::term::Clause {
                            prdcns: Cns,
                            xtor: "Nil".to_owned(),
                            context: Context::new(),
                            rhs: Rc::new(
                                core_lang::syntax::statement::Cut::new(
                                    core_lang::syntax::term::Literal::new(0),
                                    core_lang::syntax::term::XVar::covar(
                                        "a0",
                                        core_lang::syntax::types::Ty::I64,
                                    ),
                                    core_lang::syntax::types::Ty::I64,
                                )
                                .into(),
                            ),
                        },
                        core_lang::syntax::term::Clause {
                            prdcns: Cns,
                            xtor: "Cons".to_owned(),
                            context: ctx,
                            rhs: Rc::new(
                                core_lang::syntax::statement::Cut::new(
                                    core_lang::syntax::term::XVar::var(
                                        "x",
                                        core_lang::syntax::types::Ty::I64,
                                    ),
                                    core_lang::syntax::term::XVar::covar(
                                        "a0",
                                        core_lang::syntax::types::Ty::I64,
                                    ),
                                    core_lang::syntax::types::Ty::I64,
                                )
                                .into(),
                            ),
                        },
                    ],
                    ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
                },
                core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected);
    }
}
