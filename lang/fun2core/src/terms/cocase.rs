use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_context, compile_ty},
};
use core::syntax::{
    context::ContextBinding,
    term::{Cns, Prd},
    types::Ty,
};
use fun::syntax::types::OptTyped;

impl CompileWithCont for fun::syntax::terms::Cocase {
    /// ```text
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛_{c} = ⟨cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... } | c⟩
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛 = cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... }
    /// ```
    fn compile_opt(self, state: &mut CompileState, _ty: Ty) -> core::syntax::term::Term<Prd> {
        core::syntax::term::XCase {
            prdcns: Prd,
            clauses: self
                .cocases
                .into_iter()
                .map(|clause| compile_clause(clause, state))
                .collect(),
            ty: compile_ty(
                self.ty
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let ty = compile_ty(
            self.ty
                .clone()
                .expect("Types should be annotated before translation"),
        );
        core::syntax::statement::Cut {
            producer: Rc::new(self.compile_opt(state, ty.clone())),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}

fn compile_clause(
    clause: fun::syntax::terms::Clause<fun::syntax::Name>,
    state: &mut CompileState,
) -> core::syntax::Clause {
    let new_cv = state.fresh_covar();
    let ty = compile_ty(
        clause
            .get_type()
            .expect("Types should be annotated before translation"),
    );
    let mut new_context = compile_context(clause.context);
    new_context.push(ContextBinding::CovarBinding {
        covar: new_cv.clone(),
        ty: ty.clone(),
    });

    core::syntax::Clause {
        xtor: clause.xtor,
        context: new_context,
        rhs: Rc::new(
            clause.rhs.compile_with_cont(
                core::syntax::term::XVar {
                    prdcns: Cns,
                    var: new_cv,
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
    use fun::{parse_term, typing::check::Check};

    use crate::{definition::CompileWithCont, symbol_tables::table_lpair};
    use core::syntax::term::{Cns, Prd};
    use std::rc::Rc;

    #[test]
    fn compile_lpair() {
        let term = parse_term!("cocase { Fst => 1, Snd => 2 }");
        let term_typed = term
            .check(
                &table_lpair(),
                &vec![],
                &fun::syntax::types::Ty::mk_decl("LPairIntInt"),
            )
            .unwrap();
        let result = term_typed.compile_opt(
            &mut Default::default(),
            core::syntax::types::Ty::Decl("LPairIntInt".to_owned()),
        );
        let expected = core::syntax::term::XCase {
            prdcns: Prd,
            clauses: vec![
                core::syntax::Clause {
                    xtor: "Fst".to_owned(),
                    context: vec![core::syntax::context::ContextBinding::CovarBinding {
                        covar: "a0".to_owned(),
                        ty: core::syntax::types::Ty::Int(),
                    }],
                    rhs: Rc::new(
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
                },
                core::syntax::Clause {
                    xtor: "Snd".to_owned(),
                    context: vec![core::syntax::context::ContextBinding::CovarBinding {
                        covar: "a1".to_owned(),
                        ty: core::syntax::types::Ty::Int(),
                    }],
                    rhs: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 2 }.into()),
                            ty: core::syntax::types::Ty::Int(),
                            consumer: Rc::new(
                                core::syntax::term::XVar {
                                    prdcns: Cns,
                                    var: "a1".to_owned(),
                                    ty: core::syntax::types::Ty::Int(),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                },
            ],
            ty: core::syntax::types::Ty::Decl("LPairIntInt".to_owned()),
        }
        .into();
        assert_eq!(result, expected);
    }
}
