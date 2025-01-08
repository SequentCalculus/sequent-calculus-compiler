use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_ty,
};
use core_lang::syntax::{
    term::{Cns, Prd},
    types::Ty,
};

impl CompileWithCont for fun::syntax::terms::Label {
    /// ```text
    /// 〚label a {t} 〛 = μa. 〚t 〛_{a}
    /// ```
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::term::Term<Prd> {
        let var_ty = compile_ty(
            self.ty
                .expect("Types should be annotated before translation"),
        );
        let cont = core_lang::syntax::term::XVar {
            prdcns: Cns,
            var: self.label.clone(),
            ty,
        }
        .into();

        core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: self.label,
            ty: var_ty,
            statement: Rc::new(self.term.compile_with_cont(cont, state)),
        }
        .into()
    }

    /// ```text
    /// 〚label a {t} 〛_{c} = ⟨μa. 〚t 〛_{a} | c⟩
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(
            self.ty
                .clone()
                .expect("Types should be annotated before translation"),
        );
        core_lang::syntax::statement::Cut {
            producer: Rc::new(self.compile_opt(state, ty.clone())),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use fun::{parse_term, typing::check::Check};

    #[test]
    fn compile_label1() {
        let term = parse_term!("label a { 1 }");
        let term_typed = term
            .check(
                &Default::default(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::term::Mu::mu(
            "a",
            core_lang::syntax::statement::Cut::new(
                core_lang::syntax::term::Literal::new(1),
                core_lang::syntax::term::XVar::covar("a", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::types::Ty::I64,
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_label2() {
        let term = parse_term!("label a { goto(1;a) }");
        let term_typed = term
            .check(
                &Default::default(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::term::Mu::mu(
            "a",
            core_lang::syntax::statement::Cut::new(
                core_lang::syntax::term::Literal::new(1),
                core_lang::syntax::term::XVar::covar("a", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::types::Ty::I64,
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
