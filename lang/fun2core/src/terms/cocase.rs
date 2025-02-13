use crate::{
    compile::{CompileState, CompileWithCont},
    program::compile_ty,
    terms::clause::compile_coclause,
};
use core_lang::syntax::{
    terms::{Cns, Prd},
    Ty,
};

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Cocase {
    /// ```text
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛 = cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... }
    /// ```
    fn compile_opt(self, state: &mut CompileState, _ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::XCase {
            prdcns: Prd,
            clauses: self
                .cocases
                .into_iter()
                .map(|clause| compile_coclause(clause, state))
                .collect(),
            ty: compile_ty(
                self.ty
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into()
    }

    /// ```text
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛_{c} = ⟨cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... } | c⟩
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(
            self.ty
                .clone()
                .expect("Types should be annotated before translation"),
        );
        core_lang::syntax::statements::Cut {
            producer: Rc::new(self.compile_opt(state, ty.clone())),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_lpair,
        typing::check::Check,
    };

    use crate::compile::CompileWithCont;
    use core_lang::syntax::terms::Prd;
    use std::rc::Rc;

    #[test]
    fn compile_lpair() {
        let term = parse_term!("cocase { Fst => 1, Snd => 2 }");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &TypingContext::default(),
                &fun::syntax::types::Ty::mk_decl(
                    "LPair",
                    fun::syntax::types::TypeArgs::mk(vec![
                        fun::syntax::types::Ty::mk_i64(),
                        fun::syntax::types::Ty::mk_i64(),
                    ]),
                ),
            )
            .unwrap();
        let result = term_typed.compile_opt(
            &mut Default::default(),
            core_lang::syntax::types::Ty::Decl("LPair[i64, i64]".to_owned()),
        );
        let mut ctx1 = core_lang::syntax::TypingContext::default();
        ctx1.add_covar("a0", core_lang::syntax::types::Ty::I64);
        let mut ctx2 = core_lang::syntax::TypingContext::default();
        ctx2.add_covar("a1", core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::terms::XCase {
            prdcns: Prd,
            clauses: vec![
                core_lang::syntax::terms::Clause {
                    prdcns: Prd,
                    xtor: "Fst".to_owned(),
                    context: ctx1,
                    rhs: Rc::new(
                        core_lang::syntax::statements::Cut::new(
                            core_lang::syntax::terms::Literal::new(1),
                            core_lang::syntax::terms::XVar::covar(
                                "a0",
                                core_lang::syntax::types::Ty::I64,
                            ),
                            core_lang::syntax::types::Ty::I64,
                        )
                        .into(),
                    ),
                },
                core_lang::syntax::terms::Clause {
                    prdcns: Prd,
                    xtor: "Snd".to_owned(),
                    context: ctx2,
                    rhs: Rc::new(
                        core_lang::syntax::statements::Cut::new(
                            core_lang::syntax::terms::Literal::new(2),
                            core_lang::syntax::terms::XVar::covar(
                                "a1",
                                core_lang::syntax::types::Ty::I64,
                            ),
                            core_lang::syntax::types::Ty::I64,
                        )
                        .into(),
                    ),
                },
            ],
            ty: core_lang::syntax::types::Ty::Decl("LPair[i64, i64]".to_owned()),
        }
        .into();
        assert_eq!(result, expected);
    }
}
