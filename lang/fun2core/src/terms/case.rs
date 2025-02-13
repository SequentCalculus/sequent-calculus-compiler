use crate::{
    compile::{CompileState, CompileWithCont},
    program::compile_ty,
    terms::clause::compile_clause,
};
use core_lang::syntax::terms::Cns;
use fun::syntax::types::OptTyped;

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Case {
    /// ```text
    /// 〚case t of { K_1(x_11, ...) => t_1, ...} 〛_{c} = 〚t〛_{case{ K_1(x_11, ...) => 〚t_1〛_{c}, ... }}
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        // new continuation: case{ K_1(x_11,...) => 〚t_1〛_{c}, ... }
        let new_cont = core_lang::syntax::terms::XCase {
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

#[cfg(test)]
mod compile_tests {
    use crate::compile::CompileWithCont;
    use core_lang::syntax::terms::{Cns, Prd};
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_list,
        typing::check::Check,
    };
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
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let mut ctx = core_lang::syntax::TypingContext::default();
        ctx.add_var("x", core_lang::syntax::types::Ty::I64);
        ctx.add_var(
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
                            rhs: Rc::new(
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
                            context: ctx,
                            rhs: Rc::new(
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
