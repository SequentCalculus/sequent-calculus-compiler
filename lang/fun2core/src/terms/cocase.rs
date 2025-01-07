use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_context, compile_ty},
};
use core_lang::syntax::{
    context::ContextBinding,
    term::{Cns, Prd},
    types::Ty,
    Statement,
};
use fun::syntax::types::OptTyped;

impl CompileWithCont for fun::syntax::terms::Cocase {
    /// ```text
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛 = cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... }
    /// ```
    fn compile_opt(self, state: &mut CompileState, _ty: Ty) -> core_lang::syntax::term::Term<Prd> {
        core_lang::syntax::term::XCase {
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

fn compile_coclause(
    clause: fun::syntax::terms::Clause,
    state: &mut CompileState,
) -> core_lang::syntax::term::Clause<Prd, Statement> {
    let ty = compile_ty(
        clause
            .get_type()
            .expect("Types should be annotated before translation"),
    );
    let mut new_context = compile_context(clause.context);
    state.vars.extend(new_context.vars());
    state.vars.extend(new_context.covars());
    let new_covar = state.fresh_covar();

    new_context.bindings.push(ContextBinding::CovarBinding {
        covar: new_covar.clone(),
        ty: ty.clone(),
    });

    core_lang::syntax::term::Clause {
        prdcns: Prd,
        xtor: clause.xtor,
        context: new_context,
        rhs: Rc::new(
            clause.rhs.compile_with_cont(
                core_lang::syntax::term::XVar {
                    prdcns: Cns,
                    var: new_covar,
                    ty,
                }
                .into(),
                state,
            ),
        ),
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_lpair,
        typing::check::Check,
    };

    use crate::definition::CompileWithCont;
    use core_lang::syntax::{context::Context, term::Prd};
    use std::rc::Rc;

    #[test]
    fn compile_lpair() {
        let term = parse_term!("cocase { Fst => 1, Snd => 2 }");
        let term_typed = term
            .check(
                &symbol_table_lpair(),
                &TypingContext::default(),
                &fun::syntax::types::Ty::mk_decl("LPairIntInt"),
            )
            .unwrap();
        let result = term_typed.compile_opt(
            &mut Default::default(),
            core_lang::syntax::types::Ty::Decl("LPairIntInt".to_owned()),
        );
        let mut ctx1 = Context::new();
        ctx1.add_covar("a0", core_lang::syntax::types::Ty::I64);
        let mut ctx2 = Context::new();
        ctx2.add_covar("a1", core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::term::XCase {
            prdcns: Prd,
            clauses: vec![
                core_lang::syntax::term::Clause {
                    prdcns: Prd,
                    xtor: "Fst".to_owned(),
                    context: ctx1,
                    rhs: Rc::new(
                        core_lang::syntax::statement::Cut::new(
                            core_lang::syntax::term::Literal::new(1),
                            core_lang::syntax::term::XVar::covar(
                                "a0",
                                core_lang::syntax::types::Ty::I64,
                            ),
                            core_lang::syntax::types::Ty::I64,
                        )
                        .into(),
                    ),
                },
                core_lang::syntax::term::Clause {
                    prdcns: Prd,
                    xtor: "Snd".to_owned(),
                    context: ctx2,
                    rhs: Rc::new(
                        core_lang::syntax::statement::Cut::new(
                            core_lang::syntax::term::Literal::new(2),
                            core_lang::syntax::term::XVar::covar(
                                "a1",
                                core_lang::syntax::types::Ty::I64,
                            ),
                            core_lang::syntax::types::Ty::I64,
                        )
                        .into(),
                    ),
                },
            ],
            ty: core_lang::syntax::types::Ty::Decl("LPairIntInt".to_owned()),
        }
        .into();
        assert_eq!(result, expected);
    }
}
