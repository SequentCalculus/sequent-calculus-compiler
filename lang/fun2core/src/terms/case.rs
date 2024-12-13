use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_context, compile_ty},
};
use core_lang::syntax::term::Cns;
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
) -> core_lang::syntax::term::Clause {
    core_lang::syntax::term::Clause {
        xtor: clause.xtor,
        context: compile_context(clause.context),
        rhs: Rc::new(clause.rhs.compile_with_cont(cont, state)),
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::{definition::CompileWithCont, symbol_tables::table_list};
    use codespan::Span;
    use core_lang::syntax::{
        context::Context,
        term::{Cns, Prd},
    };
    use fun::{parse_term, syntax::context::TypingContext, typing::check::Check};
    use std::rc::Rc;

    #[test]
    fn compile_list() {
        let term = parse_term!("(Cons(1,Nil)).case { Nil => 0, Cons(x : Int,xs : ListInt) => x }");
        let term_typed = term
            .check(
                &table_list(),
                &TypingContext::default(),
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::Int);
        let expected = core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core_lang::syntax::types::Ty::Int,
            statement: Rc::new(
                core_lang::syntax::statement::Cut {
                    producer: Rc::new(
                        core_lang::syntax::term::Xtor {
                            prdcns: Prd,
                            id: "Cons".to_owned(),
                            args: vec![
                                core_lang::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core_lang::syntax::term::Literal { lit: 1 }.into(),
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
                        }
                        .into(),
                    ),
                    ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
                    consumer: Rc::new(
                        core_lang::syntax::term::XCase {
                            prdcns: Cns,
                            clauses: vec![
                                core_lang::syntax::term::Clause {
                                    xtor: "Nil".to_owned(),
                                    context: Context { bindings: vec![] },
                                    rhs: Rc::new(
                                        core_lang::syntax::statement::Cut {
                                            producer: Rc::new(
                                                core_lang::syntax::term::Literal { lit: 0 }.into(),
                                            ),
                                            ty: core_lang::syntax::types::Ty::Int,
                                            consumer: Rc::new(
                                                core_lang::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a0".to_owned(),
                                                    ty: core_lang::syntax::types::Ty::Int,
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                                core_lang::syntax::term::Clause {
                                    xtor: "Cons".to_owned(),
                                    context: Context {
                                        bindings: vec![
                                            core_lang::syntax::context::ContextBinding::VarBinding {
                                                var: "x".to_owned(),
                                                ty: core_lang::syntax::types::Ty::Int,
                                            },
                                            core_lang::syntax::context::ContextBinding::VarBinding {
                                                var: "xs".to_owned(),
                                                ty: core_lang::syntax::types::Ty::Decl(
                                                    "ListInt".to_owned(),
                                                ),
                                            },
                                        ],
                                    },
                                    rhs: Rc::new(
                                        core_lang::syntax::statement::Cut {
                                            producer: Rc::new(
                                                core_lang::syntax::term::XVar {
                                                    prdcns: Prd,

                                                    var: "x".to_owned(),
                                                    ty: core_lang::syntax::types::Ty::Int,
                                                }
                                                .into(),
                                            ),
                                            ty: core_lang::syntax::types::Ty::Int,

                                            consumer: Rc::new(
                                                core_lang::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a0".to_owned(),
                                                    ty: core_lang::syntax::types::Ty::Int,
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                            ],
                            ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected);
    }
}
