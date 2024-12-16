use crate::definition::{CompileState, CompileWithCont};
use core_lang::syntax::{term::Cns, types::Ty};
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::IfZ {
    /// ```text
    /// 〚IfZ(t_1, t_2, t_3) 〛_{c} = IfZ(〚t_1 〛, 〚t_2 〛_{c}, 〚t_3 〛_{c})
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statement::IfZ {
            ifc: Rc::new(self.ifc.compile_opt(state, Ty::Int)),
            thenc: Rc::new(self.thenc.compile_with_cont(cont.clone(), state)),
            elsec: Rc::new(self.elsec.compile_with_cont(cont, state)),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use fun::{parse_term, typing::check::Check};

    #[test]
    fn compile_ifz1() {
        let term = parse_term!("ifz(0,1,2)");
        let result = term.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::Int);
        let expected = core_lang::syntax::term::Mu::mu(
            "a0",
            core_lang::syntax::statement::IfZ::new(
                core_lang::syntax::term::Literal::new(0),
                core_lang::syntax::statement::Cut::new(
                    core_lang::syntax::term::Literal::new(1),
                    core_lang::syntax::term::XVar::covar("a0", core_lang::syntax::types::Ty::Int),
                    core_lang::syntax::types::Ty::Int,
                ),
                core_lang::syntax::statement::Cut::new(
                    core_lang::syntax::term::Literal::new(2),
                    core_lang::syntax::term::XVar::covar("a0", core_lang::syntax::types::Ty::Int),
                    core_lang::syntax::types::Ty::Int,
                ),
            ),
            core_lang::syntax::types::Ty::Int,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_ifz2() {
        let term = parse_term!("ifz(x,1,x)");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_int());
        let term_typed = term
            .check(&Default::default(), &ctx, &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::Int);
        let expected = core_lang::syntax::term::Mu::mu(
            "a0",
            core_lang::syntax::statement::IfZ::new(
                core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::Int),
                core_lang::syntax::statement::Cut::new(
                    core_lang::syntax::term::Literal::new(1),
                    core_lang::syntax::term::XVar::covar("a0", core_lang::syntax::types::Ty::Int),
                    core_lang::syntax::types::Ty::Int,
                ),
                core_lang::syntax::statement::Cut::new(
                    core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::Int),
                    core_lang::syntax::term::XVar::covar("a0", core_lang::syntax::types::Ty::Int),
                    core_lang::syntax::types::Ty::Int,
                ),
            ),
            core_lang::syntax::types::Ty::Int,
        )
        .into();
        assert_eq!(result, expected)
    }
}
