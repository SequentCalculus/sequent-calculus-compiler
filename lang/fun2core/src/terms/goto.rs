use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_ty,
};
use core_lang::syntax::term::Cns;
impl CompileWithCont for fun::syntax::terms::Goto {
    /// ```text
    /// 〚goto(t; a) 〛_{c} = 〚t〛_{a}
    /// ```
    fn compile_with_cont(
        self,
        _: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        self.term.compile_with_cont(
            core_lang::syntax::term::XVar {
                prdcns: Cns,
                var: self.target,
                ty: compile_ty(
                    self.ty
                        .expect("Types should be annotated before translation"),
                ),
            }
            .into(),
            state,
        )
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{parse_term, typing::check::Check};

    use crate::definition::CompileWithCont;
    use core_lang::syntax::term::{Cns, Prd};
    use std::rc::Rc;

    #[test]
    fn compile_goto1() {
        let term = parse_term!("goto(1; 'a)");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_covar("a", fun::syntax::types::Ty::mk_int());
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
                core_lang::syntax::statement::Cut::new(
                    core_lang::syntax::term::Literal::new(1),
                    core_lang::syntax::term::XVar::covar("a", core_lang::syntax::types::Ty::Int),
                    core_lang::syntax::types::Ty::Int,
                )
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_goto2() {
        let term = parse_term!("label 'a { ifz(x, goto(0;'a), x * 2) }");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_int());
        let term_typed = term
            .check(&Default::default(), &ctx, &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::Int);
        let expected = core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            ty: core_lang::syntax::types::Ty::Int,
            statement: Rc::new(
                core_lang::syntax::statement::IfZ {
                    ifc: Rc::new(
                        core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::Int)
                            .into(),
                    ),
                    thenc: Rc::new(
                        core_lang::syntax::statement::Cut::new(
                            core_lang::syntax::term::Literal::new(0),
                            core_lang::syntax::term::XVar::covar(
                                "a",
                                core_lang::syntax::types::Ty::Int,
                            ),
                            core_lang::syntax::types::Ty::Int,
                        )
                        .into(),
                    ),
                    elsec: Rc::new(
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
                            snd: Rc::new(core_lang::syntax::term::Literal::new(2).into()),
                            continuation: Rc::new(
                                core_lang::syntax::term::XVar {
                                    prdcns: Cns,
                                    var: "a".to_owned(),
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
        .into();
        assert_eq!(result, expected)
    }
}
