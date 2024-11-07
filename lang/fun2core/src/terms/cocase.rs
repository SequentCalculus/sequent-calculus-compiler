use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_context,
};
use core::syntax::{
    context::ContextBinding,
    term::{Cns, Prd},
    types::Ty,
};

impl CompileWithCont for fun::syntax::terms::Cocase {
    /// ```text
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛_{c} = ⟨cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... } | c⟩
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛 = cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... }
    /// ```
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core::syntax::term::Term<Prd> {
        core::syntax::term::XCase {
            prdcns: Prd,
            clauses: self
                .cocases
                .into_iter()
                .map(|clause| compile_clause(clause, state))
                .collect(),
            ty,
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let first_clause = self.cocases.first().unwrap();
        let ty_name = state.lookup_codata(&first_clause.xtor).unwrap().name;
        core::syntax::statement::Cut {
            producer: Rc::new(self.compile_opt(state, Ty::Decl(ty_name.clone()))),
            ty: Ty::Decl(ty_name),
            consumer: Rc::new(cont),
        }
        .into()
    }
}

fn compile_clause(
    clause: fun::syntax::terms::Clause<fun::syntax::Name>,
    state: &mut CompileState,
) -> core::syntax::Clause {
    let ty_name = state.lookup_codata(&clause.xtor).unwrap().name;
    let ty = Ty::Decl(ty_name);

    let new_cv = state.free_covar_from_state(ty.clone());

    let mut new_context = compile_context(clause.context);
    new_context.push(ContextBinding::CovarBinding {
        covar: new_cv.clone(),
        ty,
    });

    core::syntax::Clause {
        xtor: clause.xtor,
        context: new_context,
        rhs: Rc::new(
            clause.rhs.compile_with_cont(
                core::syntax::term::XVar {
                    prdcns: Cns,
                    var: new_cv,
                }
                .into(),
                state,
            ),
        ),
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::{CompileState, CompileWithCont};
    use core::syntax::{
        declaration::{Codata, TypeDeclaration, XtorSig},
        term::{Cns, Prd},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn compile_lpair() {
        let term = parse_term!("cocase { Fst => 1, Snd => 2 }");
        let mut state = CompileState::default();
        state.codata_decls.push(TypeDeclaration {
            dat: Codata,
            name: "LPairIntInt".to_owned(),
            xtors: vec![
                XtorSig {
                    xtor: Codata,
                    name: "Fst".to_owned(),
                    args: vec![],
                },
                XtorSig {
                    xtor: Codata,
                    name: "Snd".to_owned(),
                    args: vec![],
                },
            ],
        });
        let result = term.compile_opt(&mut state, Ty::Decl("LPairIntInt".to_owned()));
        let expected = core::syntax::term::XCase {
            prdcns: Prd,
            clauses: vec![
                core::syntax::Clause {
                    xtor: "Fst".to_owned(),
                    context: vec![core::syntax::context::ContextBinding::CovarBinding {
                        covar: "a0".to_owned(),
                        ty: core::syntax::types::Ty::Decl("LPairIntInt".to_owned()),
                    }],
                    rhs: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                            ty: Ty::Int(),
                            consumer: Rc::new(
                                core::syntax::term::XVar {
                                    prdcns: Cns,
                                    var: "a0".to_owned(),
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
                        ty: core::syntax::types::Ty::Decl("LPairIntInt".to_owned()),
                    }],
                    rhs: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 2 }.into()),
                            ty: Ty::Int(),
                            consumer: Rc::new(
                                core::syntax::term::XVar {
                                    prdcns: Cns,
                                    var: "a1".to_owned(),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                },
            ],
            ty: Ty::Decl("LPairIntInt".to_owned()),
        }
        .into();
        assert_eq!(result, expected);
    }
}
