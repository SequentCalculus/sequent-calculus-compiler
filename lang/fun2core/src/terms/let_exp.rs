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
        let ty_comp = compile_ty(self.var_ty);
        // new continuation: μ~x.〚t_2 〛_{c}
        let new_cont = core::syntax::term::Mu {
            prdcns: Cns,
            variable: self.variable,
            var_ty: ty_comp,
            statement: Rc::new(self.in_term.compile_with_cont(cont, state)),
        };

        // 〚t_1 〛_{new_cont}
        self.bound_term.compile_with_cont(new_cont.into(), state)
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::{CompileState, CompileWithCont};
    use core::syntax::{
        context::ContextBinding,
        declaration::{Data, TypeDeclaration, XtorSig},
        term::{Cns, Prd},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn compile_let1() {
        let term = parse_term!("let x : Int = 1 in x * x");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        core::syntax::term::Mu {
                            prdcns: Cns,
                            variable: "x".to_owned(),
                            var_ty: Ty::Int(),
                            statement: Rc::new(
                                core::syntax::statement::Op {
                                    fst: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    op: core::syntax::BinOp::Prod,
                                    snd: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    continuation: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Cns,
                                            var: "a0".to_owned(),
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
        let result = term.compile_opt(&mut st);
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
                                    prd: core::syntax::term::XVar {
                                        prdcns: Prd,
                                        var: "x".to_owned(),
                                    }
                                    .into(),
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
                        core::syntax::term::Mu {
                            prdcns: Cns,
                            variable: "x".to_owned(),
                            var_ty: Ty::Int(),
                            statement: Rc::new(
                                core::syntax::statement::Cut {
                                    producer: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    ty: Ty::Decl("ListInt".to_owned()),
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
