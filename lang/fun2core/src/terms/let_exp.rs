use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_ty,
};
use core::syntax::term::Cns;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Let {
    /// ```text
    /// 〚let x := t_1 in t_2 〛_{c} = 〚t_1 〛_{μ~x.〚t_2 〛_{c}}
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        // new continuation: μ~x.〚t_2 〛_{c}
        let new_cont = core::syntax::term::Mu {
            prdcns: Cns,
            variable: self.variable,
            ty: compile_ty(self.var_ty),
            statement: Rc::new(self.in_term.compile_with_cont(cont, state)),
        };

        // 〚t_1 〛_{new_cont}
        self.bound_term.compile_with_cont(new_cont.into(), state)
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{parse_term, typing::check::Check};

    use crate::{definition::CompileWithCont, symbol_tables::table_list};
    use core::syntax::term::{Cns, Prd};
    use std::rc::Rc;

    #[test]
    fn compile_let1() {
        let term = parse_term!("let x : Int = 1 in x * x");
        let term_typed = term
            .check(
                &Default::default(),
                &vec![],
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core::syntax::types::Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core::syntax::types::Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    ty: core::syntax::types::Ty::Int(),
                    consumer: Rc::new(
                        core::syntax::term::Mu {
                            prdcns: Cns,
                            variable: "x".to_owned(),
                            ty: core::syntax::types::Ty::Int(),
                            statement: Rc::new(
                                core::syntax::statement::Op {
                                    fst: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                            ty: core::syntax::types::Ty::Int(),
                                        }
                                        .into(),
                                    ),
                                    op: core::syntax::BinOp::Prod,
                                    snd: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                            ty: core::syntax::types::Ty::Int(),
                                        }
                                        .into(),
                                    ),
                                    continuation: Rc::new(
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
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_let2() {
        let term = parse_term!("let x : ListInt = Cons(x,Nil) in x");
        let term_typed = term
            .check(
                &table_list(),
                &vec![fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                }],
                &fun::syntax::types::Ty::mk_decl("ListInt"),
            )
            .unwrap();
        let result = term_typed.compile_opt(
            &mut Default::default(),
            core::syntax::types::Ty::Decl("ListInt".to_owned()),
        );
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Prd,
                            id: "Cons".to_owned(),
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::term::XVar {
                                        prdcns: Prd,
                                        var: "x".to_owned(),
                                        ty: core::syntax::types::Ty::Int(),
                                    }
                                    .into(),
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
                        core::syntax::term::Mu {
                            prdcns: Cns,
                            variable: "x".to_owned(),
                            ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                            statement: Rc::new(
                                core::syntax::statement::Cut {
                                    producer: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                            ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                                        }
                                        .into(),
                                    ),
                                    ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                                    consumer: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Cns,
                                            var: "a0".to_owned(),
                                            ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
