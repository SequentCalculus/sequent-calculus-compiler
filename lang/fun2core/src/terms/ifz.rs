use crate::compile::{CompileState, CompileWithCont};
use core_lang::syntax::{terms::Cns, Ty};

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::IfZ {
    /// ```text
    /// 〚IfZ(t_1) {t_2} else {t_3} 〛_{c} = IfZ(〚t_1 〛, 〚t_2 〛_{c}, 〚t_3 〛_{c})
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statements::IfZ {
            sort: match self.sort {
                fun::syntax::terms::IfZSort::Equal => core_lang::syntax::statements::IfZSort::Equal,
                fun::syntax::terms::IfZSort::NotEqual => {
                    core_lang::syntax::statements::IfZSort::NotEqual
                }
            },
            ifc: Rc::new(self.ifc.compile_opt(state, Ty::I64)),
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
    fn compile_ifz1() {
        let term = parse_term!("if 0 == 0 {1} else {2}");
        let result = term.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::IfZ::new(
                core_lang::syntax::terms::Literal::new(0),
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
    fn compile_ifz2() {
        let term = parse_term!("if x == 0 {1} else {x}");
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
            core_lang::syntax::statements::IfZ::new(
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
