use crate::definition::{CompileState, CompileWithCont};
use core_lang::syntax::{term::Cns, types::Ty};
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::IfC {
    /// ```text
    /// 〚IfE(t_1, t_2, t_3, t_4) 〛_{c} = IfE(〚t_1 〛, 〚t_2 〛, 〚t_3 〛_{c}, 〚t_4 〛_{c})
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statement::IfC {
            sort: match self.sort {
                fun::syntax::terms::IfSort::Equal => core_lang::syntax::statement::IfSort::Equal,
                fun::syntax::terms::IfSort::Less => core_lang::syntax::statement::IfSort::Less,
            },
            fst: Rc::new(self.fst.compile_opt(state, Ty::Int)),
            snd: Rc::new(self.snd.compile_opt(state, Ty::Int)),
            thenc: Rc::new(self.thenc.compile_with_cont(cont.clone(), state)),
            elsec: Rc::new(self.elsec.compile_with_cont(cont, state)),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use std::rc::Rc;

    use fun::{parse_term, typing::check::Check};

    use crate::definition::CompileWithCont;
    use core_lang::syntax::term::Prd;

    #[test]
    fn compile_ife1() {
        let term = parse_term!("ife(0,1,1,2)");
        let result = term.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::Int);
        let expected = core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core_lang::syntax::types::Ty::Int,
            statement: Rc::new(
                core_lang::syntax::statement::IfC {
                    sort: core_lang::syntax::statement::IfSort::Equal,
                    fst: Rc::new(core_lang::syntax::term::Literal::new(0).into()),
                    snd: Rc::new(core_lang::syntax::term::Literal::new(1).into()),
                    thenc: Rc::new(
                        core_lang::syntax::statement::Cut::new(
                            core_lang::syntax::term::Literal::new(1),
                            core_lang::syntax::term::XVar::covar(
                                "a0",
                                core_lang::syntax::types::Ty::Int,
                            ),
                            core_lang::syntax::types::Ty::Int,
                        )
                        .into(),
                    ),
                    elsec: Rc::new(
                        core_lang::syntax::statement::Cut::new(
                            core_lang::syntax::term::Literal::new(2),
                            core_lang::syntax::term::XVar::covar(
                                "a0",
                                core_lang::syntax::types::Ty::Int,
                            ),
                            core_lang::syntax::types::Ty::Int,
                        )
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
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_int());
        let term_typed = term
            .check(&Default::default(), &ctx, &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::Int);
        let expected = core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core_lang::syntax::types::Ty::Int,
            statement: Rc::new(
                core_lang::syntax::statement::IfC {
                    sort: core_lang::syntax::statement::IfSort::Equal,
                    fst: Rc::new(
                        core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::Int)
                            .into(),
                    ),
                    snd: Rc::new(
                        core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::Int)
                            .into(),
                    ),
                    thenc: Rc::new(
                        core_lang::syntax::statement::Cut::new(
                            core_lang::syntax::term::Literal::new(1),
                            core_lang::syntax::term::XVar::covar(
                                "a0",
                                core_lang::syntax::types::Ty::Int,
                            ),
                            core_lang::syntax::types::Ty::Int,
                        )
                        .into(),
                    ),
                    elsec: Rc::new(
                        core_lang::syntax::statement::Cut::new(
                            core_lang::syntax::term::XVar::var(
                                "x",
                                core_lang::syntax::types::Ty::Int,
                            ),
                            core_lang::syntax::term::XVar::covar(
                                "a0",
                                core_lang::syntax::types::Ty::Int,
                            ),
                            core_lang::syntax::types::Ty::Int,
                        )
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
