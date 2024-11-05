use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_context,
};
use core::syntax::{term::Cns, types::Ty};

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
        let ty_name = state
            .lookup_data(&self.cases.first().unwrap().xtor)
            .unwrap()
            .name;
        let new_cont = core::syntax::term::XCase {
            prdcns: Cns,
            clauses: self
                .cases
                .into_iter()
                .map(|clause| compile_clause(clause, cont.clone(), state))
                .collect(),
            ty: Ty::Decl(ty_name),
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
    use crate::definition::{CompileState, CompileWithCont};
    use core::syntax::{
        context::ContextBinding,
        declaration::{Data, TypeDeclaration, XtorSig},
        term::{Cns, Prd},
        types::Ty,
    };
    use fun::{
        parse_term,
        typing::{
            check::terms::Check,
            symbol_table::{Polarity, SymbolTable},
        },
    };
    use std::rc::Rc;

    #[test]
    fn compile_list() {
        let term = parse_term!("(Cons(1,Nil)).case { Nil => 0, Cons(x : Int,xs : ListInt) => x }");
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        symbol_table.ctors.insert("Nil".to_owned(), vec![]);
        symbol_table.ctors.insert(
            "Cons".to_owned(),
            vec![
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                },
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: fun::syntax::types::Ty::mk_decl("ListInt"),
                },
            ],
        );
        let term_typed = term
            .check(&symbol_table, &vec![], &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let mut st = CompileState::default();
        st.data_decls.push(TypeDeclaration {
            dat: Data,
            name: "ListInt".to_owned(),
            xtors: vec![
                XtorSig {
                    xtor: Data,
                    name: "Nil".to_owned(),
                    args: vec![],
                },
                XtorSig {
                    xtor: Data,
                    name: "Cons".to_owned(),
                    args: vec![
                        ContextBinding::VarBinding {
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::VarBinding {
                            var: "xs".to_owned(),
                            ty: Ty::Decl("ListInt".to_owned()),
                        },
                    ],
                },
            ],
        });
        let result = term_typed.compile_opt(&mut st);
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Prd,
                            id: "Cons".to_owned(),
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                                    prd: core::syntax::term::Literal { lit: 1 }.into(),
                                    ty: Ty::Int(),
                                },
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                                    prd: core::syntax::term::Xtor {
                                        prdcns: Prd,
                                        id: "Nil".to_owned(),
                                        args: vec![],
                                        ty: Ty::Decl("ListInt".to_owned()),
                                    }
                                    .into(),
                                    ty: Ty::Decl("ListInt".to_owned()),
                                },
                            ],
                            ty: Ty::Decl("ListInt".to_owned()),
                        }
                        .into(),
                    ),
                    ty: Ty::Decl("ListInt".to_owned()),
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
                                            ty: Ty::Int(),
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a0".to_owned(),
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
                                            ty: Ty::Int(),
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
                                                }
                                                .into(),
                                            ),
                                            ty: Ty::Int(),
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a0".to_owned(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                            ],
                            ty: Ty::Decl("ListInt".to_owned()),
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
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "TupIntInt".to_owned(),
            (Polarity::Data, vec!["Tup".to_owned()]),
        );
        symbol_table.ctors.insert(
            "Tup".to_owned(),
            vec![
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                },
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "y".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                },
            ],
        );
        let term_typed = term
            .check(&symbol_table, &vec![], &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let mut state = CompileState::default();
        state.data_decls.push(TypeDeclaration {
            dat: Data,
            name: "TupIntInt".to_owned(),
            xtors: vec![XtorSig {
                xtor: Data,
                name: "Tup".to_owned(),
                args: vec![
                    ContextBinding::VarBinding {
                        var: "x".to_owned(),
                        ty: Ty::Int(),
                    },
                    ContextBinding::VarBinding {
                        var: "y".to_owned(),
                        ty: Ty::Int(),
                    },
                ],
            }],
        });
        let result = term_typed.compile_opt(&mut state);
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Prd,
                            id: "Tup".to_owned(),
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                                    prd: core::syntax::term::Literal { lit: 1 }.into(),
                                    ty: Ty::Int(),
                                },
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                                    prd: core::syntax::term::Literal { lit: 2 }.into(),
                                    ty: Ty::Int(),
                                },
                            ],
                            ty: Ty::Decl("TupIntInt".to_owned()),
                        }
                        .into(),
                    ),
                    ty: Ty::Decl("TupIntInt".to_owned()),
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
                                            }
                                            .into(),
                                        ),
                                        ty: Ty::Int(),
                                        consumer: Rc::new(
                                            core::syntax::term::XVar {
                                                prdcns: Cns,
                                                var: "a0".to_owned(),
                                            }
                                            .into(),
                                        ),
                                    }
                                    .into(),
                                ),
                            }],
                            ty: Ty::Decl("TupIntInt".to_owned()),
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
