use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_subst, compile_ty},
};
use core_lang::syntax::{
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
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let mut new_args = compile_subst(self.args, state);
        new_args.push(core_lang::syntax::substitution::SubstitutionBinding::ConsumerBinding(cont));
        core_lang::syntax::statement::Fun {
            name: self.name,
            args: new_args,
            ty: compile_ty(
                self.ret_ty
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into()
    }

    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::term::Term<Prd> {
        state.covars.extend(subst_covars(&self.args));
        // default implementation
        let new_covar = state.fresh_covar();
        let var_ty = compile_ty(
            self.ret_ty
                .clone()
                .expect("Types should be annotated before translation"),
        );
        let new_statement = self.compile_with_cont(
            core_lang::syntax::term::XVar {
                prdcns: Cns,
                var: new_covar.clone(),
                ty: var_ty,
            }
            .into(),
            state,
        );
        core_lang::syntax::term::Mu {
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
    use codespan::Span;
    use fun::{
        parse_term,
        syntax::context::{ContextBinding, TypingContext},
        typing::{check::Check, symbol_table::SymbolTable},
    };

    use crate::definition::CompileWithCont;
    use core_lang::syntax::term::{Cns, Prd};
    use std::{collections::HashMap, rc::Rc};

    #[test]
    fn compile_fac() {
        let term = parse_term!("fac(3)");
        let term_typed = term
            .check(
                &{
                    let mut funs = HashMap::new();
                    funs.insert(
                        "fac".to_owned(),
                        (
                            TypingContext {
                                span: Span::default(),
                                bindings: vec![ContextBinding::TypedVar {
                                    var: "x".to_owned(),
                                    ty: fun::syntax::types::Ty::mk_int(),
                                }],
                            },
                            fun::syntax::types::Ty::mk_int(),
                        ),
                    );

                    SymbolTable {
                        ctors: HashMap::new(),
                        dtors: HashMap::new(),
                        funs,
                        ty_ctors: HashMap::new(),
                    }
                },
                &fun::syntax::context::TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::Int);
        let expected = core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core_lang::syntax::types::Ty::Int,
            statement: Rc::new(
                core_lang::syntax::statement::Fun {
                    name: "fac".to_owned(),
                    args: vec![
                        core_lang::syntax::substitution::SubstitutionBinding::ProducerBinding(
                            core_lang::syntax::term::Literal { lit: 3 }.into(),
                        ),
                        core_lang::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                            core_lang::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                                ty: core_lang::syntax::types::Ty::Int,
                            }
                            .into(),
                        ),
                    ],
                    ty: core_lang::syntax::types::Ty::Int,
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
