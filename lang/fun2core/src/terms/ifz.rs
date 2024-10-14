use crate::definition::{CompileState, CompileWithCont};
use core::syntax::term::Cns;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::IfZ {
    /// ```text
    /// 〚IfZ(t_1, t_2, t_3) 〛_{c} = IfZ(〚t_1 〛, 〚t_2 〛_{c}, 〚t_3 〛_{c})
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::statement::IfZ {
            ifc: Rc::new(self.ifc.compile_opt(state)),
            thenc: Rc::new(self.thenc.compile_with_cont(cont.clone(), state)),
            elsec: Rc::new(self.elsec.compile_with_cont(cont, state)),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {

    use std::rc::Rc;

    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use core::syntax::term::{Cns, Prd};

    #[test]
    fn compile_ifz1() {
        let term = parse_term!("ifz(0,1,2)");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::statement::IfZ {
                    ifc: Rc::new(core::syntax::term::Literal { lit: 0 }.into()),
                    thenc: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
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
                    elsec: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 2 }.into()),
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
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_ifz2() {
        let term = parse_term!("ifz(x,1,x)");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::statement::IfZ {
                    ifc: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    thenc: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
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
                    elsec: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(
                                core::syntax::term::XVar {
                                    prdcns: Prd,
                                    var: "x".to_owned(),
                                }
                                .into(),
                            ),
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
        .into();
        assert_eq!(result, expected)
    }
}
