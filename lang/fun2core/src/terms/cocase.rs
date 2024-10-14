use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_context,
};
use core::syntax::{context::ContextBinding, types::Ty};

impl CompileWithCont for fun::syntax::terms::Cocase {
    /// ```text
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛_{c} = ⟨cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... } | c⟩
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛 = cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... }
    /// ```
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        core::syntax::Cocase {
            cocases: self
                .cocases
                .into_iter()
                .map(|clause| compile_clause(clause, state))
                .collect(),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::statement::Cut {
            producer: Rc::new(self.compile_opt(state)),
            consumer: Rc::new(cont),
        }
        .into()
    }
}

fn compile_clause(
    clause: fun::syntax::terms::Clause<fun::syntax::Name>,
    state: &mut CompileState,
) -> core::syntax::Clause {
    let new_cv = state.free_covar_from_state();
    let mut new_context = compile_context(clause.context);
    //what should be the type here
    new_context.push(ContextBinding::CovarBinding {
        covar: new_cv.clone(),
        //TODO: this type needs to be looked up in the codata declaration
        ty: Ty::Int(),
    });

    core::syntax::Clause {
        xtor: clause.xtor,
        context: new_context,
        rhs: Rc::new(
            clause
                .rhs
                .compile_with_cont(core::syntax::Covariable { covar: new_cv }.into(), state),
        ),
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    #[test]
    fn complie_lpair() {
        let term = parse_term!("cocase { Fst => 1, Snd => 2 }");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Cocase {
            cocases: vec![
                core::syntax::Clause {
                    xtor: "Fst".to_owned(),
                    context: vec![core::syntax::context::ContextBinding::CovarBinding {
                        covar: "a0".to_owned(),
                        ty: core::syntax::types::Ty::Int(),
                    }],
                    rhs: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                            consumer: Rc::new(
                                core::syntax::Covariable {
                                    covar: "a0".to_owned(),
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
                            producer: Rc::new(core::syntax::Literal { lit: 2 }.into()),
                            consumer: Rc::new(
                                core::syntax::Covariable {
                                    covar: "a1".to_owned(),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                },
            ],
        }
        .into();
        assert_eq!(result, expected);
    }
}
