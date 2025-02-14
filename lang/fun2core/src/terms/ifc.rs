use crate::compile::{CompileState, CompileWithCont};
use core_lang::syntax::{terms::Cns, Ty};

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::IfC {
    /// ```text
    /// 〚IfC(t_1, t_2) {t_3} else {t_4} 〛_{c} = IfC(〚t_1 〛, 〚t_2 〛, 〚t_3 〛_{c}, 〚t_4 〛_{c})
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statements::IfC {
            sort: match self.sort {
                fun::syntax::terms::IfSort::Equal => core_lang::syntax::statements::IfSort::Equal,
                fun::syntax::terms::IfSort::NotEqual => {
                    core_lang::syntax::statements::IfSort::NotEqual
                }
                fun::syntax::terms::IfSort::Less => core_lang::syntax::statements::IfSort::Less,
                fun::syntax::terms::IfSort::LessOrEqual => {
                    core_lang::syntax::statements::IfSort::LessOrEqual
                }
            },
            fst: Rc::new(self.fst.compile_opt(state, Ty::I64)),
            snd: Rc::new(self.snd.compile_opt(state, Ty::I64)),
            thenc: Rc::new(self.thenc.compile_with_cont(cont.clone(), state)),
            elsec: Rc::new(self.elsec.compile_with_cont(cont, state)),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::CompileWithCont;
    use fun::{parse_term, typing::check::Check};

    #[test]
    fn compile_ife1() {
        let term = parse_term!("if 3 == 4 {1} else {2}");
        let result = term.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::IfC::ife(
                core_lang::syntax::terms::Literal::new(3),
                core_lang::syntax::terms::Literal::new(4),
                core_lang::syntax::statements::Cut::new(
                    core_lang::syntax::terms::Literal::new(1),
                    core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::statements::Cut::new(
                    core_lang::syntax::terms::Literal::new(2),
                    core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_ife2() {
        let term = parse_term!("if x == x {1} else {x}");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut Default::default(),
                &ctx,
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::IfC::ife(
                core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::statements::Cut::new(
                    core_lang::syntax::terms::Literal::new(1),
                    core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::statements::Cut::new(
                    core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
