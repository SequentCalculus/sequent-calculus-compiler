use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_ty,
};
use core_lang::syntax::term::Cns;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Let {
    /// ```text
    /// 〚let x := t_1 in t_2 〛_{c} = <〚t_1 〛| μ~x.〚t_2 〛_{c}> if t_1: codata {...}
    /// 〚let x := t_1 in t_2 〛_{c} = 〚t_1 〛_{μ~x.〚t_2 〛_{c}}
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(self.var_ty);
        // new continuation: μ~x.〚t_2 〛_{c}
        let new_cont = core_lang::syntax::term::Mu {
            prdcns: Cns,
            variable: self.variable,
            ty: ty.clone(),
            statement: Rc::new(self.in_term.compile_with_cont(cont, state)),
        }
        .into();

        if ty.is_codata(state.codata_types) {
            // <〚t_1 〛| new_cont>
            core_lang::syntax::statement::Cut {
                producer: Rc::new(self.bound_term.compile_opt(state, ty.clone())),
                ty,
                consumer: Rc::new(new_cont),
            }
            .into()
        } else {
            // 〚t_1 〛_{new_cont}
            self.bound_term.compile_with_cont(new_cont, state)
        }
    }
}

#[cfg(test)]
mod compile_tests {
    use codespan::Span;
    use fun::{parse_term, typing::check::Check};

    use crate::{definition::CompileWithCont, symbol_tables::table_list};
    use core_lang::syntax::term::{Cns, Prd};
    use std::rc::Rc;

    #[test]
    fn compile_let1() {
        let term = parse_term!("let x : Int = 1 in x * x");
        let term_typed = term
            .check(
                &Default::default(),
                &fun::syntax::context::TypingContext::default(),
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
                    producer: Rc::new(core_lang::syntax::term::Literal { lit: 1 }.into()),
                    ty: core_lang::syntax::types::Ty::Int,
                    consumer: Rc::new(
                        core_lang::syntax::term::Mu {
                            prdcns: Cns,
                            variable: "x".to_owned(),
                            ty: core_lang::syntax::types::Ty::Int,
                            statement: Rc::new(
                                core_lang::syntax::statement::Op {
                                    fst: Rc::new(
                                        core_lang::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                            ty: core_lang::syntax::types::Ty::Int,
                                        }
                                        .into(),
                                    ),
                                    op: core_lang::syntax::BinOp::Prod,
                                    snd: Rc::new(
                                        core_lang::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                            ty: core_lang::syntax::types::Ty::Int,
                                        }
                                        .into(),
                                    ),
                                    continuation: Rc::new(
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
                &fun::syntax::context::TypingContext {
                    span: Span::default(),
                    bindings: vec![fun::syntax::context::ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: fun::syntax::types::Ty::mk_int(),
                    }],
                },
                &fun::syntax::types::Ty::mk_decl("ListInt"),
            )
            .unwrap();
        let result = term_typed.compile_opt(
            &mut Default::default(),
            core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
        );
        let expected = core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
            statement: Rc::new(
                core_lang::syntax::statement::Cut {
                    producer: Rc::new(
                        core_lang::syntax::term::Xtor {
                            prdcns: Prd,
                            id: "Cons".to_owned(),
                            args: vec![
                                core_lang::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core_lang::syntax::term::XVar {
                                        prdcns: Prd,
                                        var: "x".to_owned(),
                                        ty: core_lang::syntax::types::Ty::Int,
                                    }
                                    .into(),
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
                        core_lang::syntax::term::Mu {
                            prdcns: Cns,
                            variable: "x".to_owned(),
                            ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
                            statement: Rc::new(
                                core_lang::syntax::statement::Cut {
                                    producer: Rc::new(
                                        core_lang::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                            ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
                                        }
                                        .into(),
                                    ),
                                    ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
                                    consumer: Rc::new(
                                        core_lang::syntax::term::XVar {
                                            prdcns: Cns,
                                            var: "a0".to_owned(),
                                            ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
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
