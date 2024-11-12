use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_subst, compile_ty},
};
use core::syntax::{
    term::{Cns, Prd},
    types::Ty,
};
use fun::syntax::substitution::subst_covars;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Fun {
    /// ```text
    /// 〚f(t_1, ...; a_1, ...) 〛_{c} = f(〚t_1〛, ...; a_1, ..., c)
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let mut new_args = compile_subst(self.args, state);
        new_args.push(core::syntax::substitution::SubstitutionBinding::ConsumerBinding(cont));
        core::syntax::statement::Fun {
            name: self.name,
            args: new_args,
            ty: compile_ty(self.ret_ty.unwrap()),
        }
        .into()
    }

    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core::syntax::term::Term<Prd> {
        state.covars.extend(subst_covars(&self.args));
        // default implementation
        let new_covar = state.free_covar_from_state();
        let var_ty = compile_ty(self.ret_ty.clone().unwrap());
        let new_statement = self.compile_with_cont(
            core::syntax::term::XVar {
                prdcns: Cns,
                var: new_covar.clone(),
                ty: var_ty,
            }
            .into(),
            state,
        );
        core::syntax::term::Mu {
            prdcns: Prd,
            variable: new_covar,
            ty,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use core::syntax::term::{Cns, Prd};
    use std::rc::Rc;

    #[test]
    fn compile_fac() {
        let term = parse_term!("fac(3)");
        let result = term.compile_opt(&mut Default::default(), core::syntax::types::Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core::syntax::types::Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Fun {
                    name: "fac".to_owned(),
                    args: vec![
                        core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                            core::syntax::term::Literal { lit: 3 }.into(),
                        ),
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                            core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                                ty: core::syntax::types::Ty::Int(),
                            }
                            .into(),
                        ),
                    ],
                    ty: core::syntax::types::Ty::Int(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_swap() {
        let term = parse_term!("swap(Tup(1,2))");
        let result = term.compile_opt(
            &mut Default::default(),
            core::syntax::types::Ty::Decl("TupIntInt".to_owned()),
        );
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core::syntax::types::Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Fun {
                    name: "swap".to_owned(),
                    args: vec![
                        core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                            core::syntax::term::Xtor {
                                prdcns: Prd,
                                id: "Tup".to_owned(),
                                args: vec![
                            core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                core::syntax::term::Literal { lit: 1 }.into(),
                            ),
                            core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                core::syntax::term::Literal { lit: 2 }.into(),
                            ),
                        ],
                                ty: core::syntax::types::Ty::Decl("TupIntInt".to_owned()),
                            }
                            .into(),
                        ),
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                            core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                                ty: core::syntax::types::Ty::Int(),
                            }
                            .into(),
                        ),
                    ],
                    ty: core::syntax::types::Ty::Int(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_multfast() {
        let term = parse_term!("multFast(Nil, 'a0)");
        let result = term.compile_opt(&mut Default::default(), core::syntax::types::Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a1".to_owned(),
            ty: core::syntax::types::Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Fun {
                    name: "multFast".to_owned(),
                    args: vec![
                        core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                            core::syntax::term::Xtor {
                                prdcns: Prd,
                                id: "Nil".to_owned(),
                                args: vec![],
                                ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                            }
                            .into(),
                        ),
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                            core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                                ty: core::syntax::types::Ty::Int(),
                            }
                            .into(),
                        ),
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                            core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a1".to_owned(),
                                ty: core::syntax::types::Ty::Int(),
                            }
                            .into(),
                        ),
                    ],
                    ty: core::syntax::types::Ty::Int(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
