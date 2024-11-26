use crate::definition::{CompileState, CompileWithCont};
use core::syntax::{term::Cns, types::Ty};
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::IfC {
    /// ```text
    /// 〚IfE(t_1, t_2, t_3, t_4) 〛_{c} = IfE(〚t_1 〛, 〚t_2 〛, 〚t_3 〛_{c}, 〚t_4 〛_{c})
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        match self.sort {
            fun::syntax::terms::IfSort::Equal => core::syntax::statement::IfC {
                sort: core::syntax::statement::ifc::IfSort::Equal,
                fst: Rc::new(self.fst.compile_opt(state, Ty::Int())),
                snd: Rc::new(self.snd.compile_opt(state, Ty::Int())),
                thenc: Rc::new(self.thenc.compile_with_cont(cont.clone(), state)),
                elsec: Rc::new(self.elsec.compile_with_cont(cont, state)),
            }
            .into(),
            fun::syntax::terms::IfSort::Less => core::syntax::statement::IfC {
                sort: core::syntax::statement::ifc::IfSort::Equal,
                fst: Rc::new(self.fst.compile_opt(state, Ty::Int())),
                snd: Rc::new(self.snd.compile_opt(state, Ty::Int())),
                thenc: Rc::new(self.thenc.compile_with_cont(cont.clone(), state)),
                elsec: Rc::new(self.elsec.compile_with_cont(cont, state)),
            }
            .into(),
        }
    }
}

#[cfg(test)]
mod compile_tests {

    use std::rc::Rc;

    use fun::{parse_term, typing::check::Check};

    use crate::definition::CompileWithCont;
    use core::syntax::term::{Cns, Prd};

    #[test]
    fn compile_ife1() {
        let term = parse_term!("ife(0,1,1,2)");
        let result = term.compile_opt(&mut Default::default(), core::syntax::types::Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core::syntax::types::Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::IfC {
                    sort: core::syntax::statement::IfSort::Equal,
                    fst: Rc::new(core::syntax::term::Literal { lit: 0 }.into()),
                    snd: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    thenc: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
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
                    elsec: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 2 }.into()),
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
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_ife2() {
        let term = parse_term!("ife(x,x,1,x)");
        let term_typed = term
            .check(
                &Default::default(),
                &fun::syntax::context::TypingContext {
                    bindings: vec![fun::syntax::context::ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: fun::syntax::types::Ty::mk_int(),
                    }],
                },
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
                core::syntax::statement::IfC {
                    sort: core::syntax::statement::ifc::IfSort::Equal,
                    fst: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                            ty: core::syntax::types::Ty::Int(),
                        }
                        .into(),
                    ),
                    snd: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                            ty: core::syntax::types::Ty::Int(),
                        }
                        .into(),
                    ),
                    thenc: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
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
                    elsec: Rc::new(
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
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
