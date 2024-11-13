use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_context, compile_ty},
};
use core::syntax::term::Cns;

impl CompileWithCont for fun::syntax::terms::Case {
    /// ```text
    /// 〚case t of { K_1(x_11, ...) => t_1, ...} 〛_{c} = 〚t〛_{case{ K_1(x_11, ...) => 〚t_1〛_{c}, ... }}
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        // new continuation: case{ K_1(x_11,...) => 〚t_1〛_{c}, ... }
        let new_cont = core::syntax::term::XCase {
            prdcns: Cns,
            clauses: self
                .cases
                .into_iter()
                .map(|clause| compile_clause(clause, cont.clone(), state))
                .collect(),
            ty: compile_ty(self.ty.unwrap()),
        }
        .into();

        // 〚t〛_{new_cont}
        Rc::unwrap_or_clone(self.destructee).compile_with_cont(new_cont, state)
    }
}

fn compile_clause(
    clause: fun::syntax::terms::Clause<fun::syntax::Name>,
    cont: core::syntax::term::Term<Cns>,
    state: &mut CompileState,
) -> core::syntax::Clause {
    core::syntax::Clause {
        xtor: clause.xtor,
        context: compile_context(clause.context),
        rhs: Rc::new(clause.rhs.compile_with_cont(cont, state)),
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::{
        definition::CompileWithCont,
        symbol_tables::{table_list, table_tup},
    };
    use core::syntax::term::{Cns, Prd};
    use fun::{parse_term, typing::check::terms::Check};
    use std::rc::Rc;

    #[test]
    fn compile_list() {
        let term = parse_term!("(Cons(1,Nil)).case { Nil => 0, Cons(x : Int,xs : ListInt) => x }");
        let term_typed = term
            .check(&table_list(), &vec![], &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core::syntax::types::Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core::syntax::types::Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Prd,
                            id: "Cons".to_owned(),
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::term::Literal { lit: 1 }.into(),
                                ),
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::term::Xtor {
                                        prdcns: Prd,
                                        id: "Nil".to_owned(),
                                        args: vec![],
                                        ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                                    }
                                    .into(),
                                ),
                            ],
                            ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                        }
                        .into(),
                    ),
                    ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                    consumer: Rc::new(
                        core::syntax::term::XCase {
                            prdcns: Cns,
                            clauses: vec![
                                core::syntax::Clause {
                                    xtor: "Nil".to_owned(),
                                    context: vec![],
                                    rhs: Rc::new(
                                        core::syntax::statement::Cut {
                                            producer: Rc::new(
                                                core::syntax::term::Literal { lit: 0 }.into(),
                                            ),
                                            ty: core::syntax::types::Ty::Int(),
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a0".to_owned(),
                                                    ty: core::syntax::types::Ty::Int(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                                core::syntax::Clause {
                                    xtor: "Cons".to_owned(),
                                    context: vec![
                                        core::syntax::context::ContextBinding::VarBinding {
                                            var: "x".to_owned(),
                                            ty: core::syntax::types::Ty::Int(),
                                        },
                                        core::syntax::context::ContextBinding::VarBinding {
                                            var: "xs".to_owned(),
                                            ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                                        },
                                    ],
                                    rhs: Rc::new(
                                        core::syntax::statement::Cut {
                                            producer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Prd,

                                                    var: "x".to_owned(),
                                                    ty: core::syntax::types::Ty::Int(),
                                                }
                                                .into(),
                                            ),
                                            ty: core::syntax::types::Ty::Int(),

                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a0".to_owned(),
                                                    ty: core::syntax::types::Ty::Int(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                            ],
                            ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
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

    #[test]
    fn compile_tup() {
        let term = parse_term!("(Tup(1,2)).case { Tup(x: Int, y: Int) => y }");
        let term_typed = term
            .check(&table_tup(), &vec![], &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core::syntax::types::Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core::syntax::types::Ty::Int(),

            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Prd,
                            id: "Tup".to_owned(),
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::term::Literal { lit: 1 }.into(),
                                ),
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::term::Literal { lit: 2 }.into(),
                                ),
                            ],
                            ty: core::syntax::types::Ty::Decl("TupIntInt".to_owned()),
                        }
                        .into(),
                    ),
                    ty: core::syntax::types::Ty::Decl("TupIntInt".to_owned()),
                    consumer: Rc::new(
                        core::syntax::term::XCase {
                            prdcns: Cns,
                            clauses: vec![core::syntax::Clause {
                                xtor: "Tup".to_owned(),
                                context: vec![
                                    core::syntax::context::ContextBinding::VarBinding {
                                        var: "x".to_owned(),
                                        ty: core::syntax::types::Ty::Int(),
                                    },
                                    core::syntax::context::ContextBinding::VarBinding {
                                        var: "y".to_owned(),
                                        ty: core::syntax::types::Ty::Int(),
                                    },
                                ],
                                rhs: Rc::new(
                                    core::syntax::statement::Cut {
                                        producer: Rc::new(
                                            core::syntax::term::XVar {
                                                prdcns: Prd,
                                                var: "y".to_owned(),
                                                ty: core::syntax::types::Ty::Int(),
                                            }
                                            .into(),
                                        ),
                                        ty: core::syntax::types::Ty::Int(),

                                        consumer: Rc::new(
                                            core::syntax::term::XVar {
                                                prdcns: Cns,
                                                var: "a0".to_owned(),
                                                ty: core::syntax::types::Ty::Int(),
                                            }
                                            .into(),
                                        ),
                                    }
                                    .into(),
                                ),
                            }],
                            ty: core::syntax::types::Ty::Decl("TupIntInt".to_owned()),
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
