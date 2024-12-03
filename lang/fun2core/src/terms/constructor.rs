use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_subst, compile_ty},
};
use core_lang::syntax::{
    term::{Cns, Prd},
    types::Ty,
};
use fun::syntax::substitution::subst_covars;

impl CompileWithCont for fun::syntax::terms::Constructor {
    /// ```text
    /// 〚K(t_1, ...) 〛_{c} = ⟨K( 〚t_1〛, ...) | c⟩
    /// 〚K(t_1, ...) 〛 = K( 〚t_1〛, ...)
    /// ```
    fn compile_opt(self, state: &mut CompileState, _ty: Ty) -> core_lang::syntax::term::Term<Prd> {
        state.covars.extend(subst_covars(&self.args));
        core_lang::syntax::term::Xtor {
            prdcns: Prd,
            id: self.id,
            args: compile_subst(self.args, state),
            ty: compile_ty(
                self.ty
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into()
    }

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
    use codespan::Span;
    use fun::{parse_term, syntax::context::TypingContext, typing::check::Check};

    use crate::{definition::CompileWithCont, symbol_tables::table_list};
    use core_lang::syntax::term::Prd;

    #[test]
    fn compile_cons() {
        let term = parse_term!("Cons(1,Nil)");
        let term_typed = term
            .check(
                &table_list(),
                &TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                &fun::syntax::types::Ty::mk_decl("ListInt"),
            )
            .unwrap();
        let result = term_typed.compile_opt(
            &mut Default::default(),
            core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
        );
        let expected = core_lang::syntax::term::Xtor {
            prdcns: Prd,
            id: "Cons".to_owned(),
            args: vec![
                core_lang::syntax::substitution::SubstitutionBinding::ProducerBinding(
                    core_lang::syntax::term::Literal { lit: 1 }.into(),
                ),
                core_lang::syntax::substitution::SubstitutionBinding::ProducerBinding(
                    core_lang::syntax::term::Xtor {
                        prdcns: Prd,
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
                    }
                    .into(),
                ),
            ],
            ty: core_lang::syntax::types::Ty::Decl("ListInt".to_owned()),
        }
        .into();
        assert_eq!(result, expected)
    }
}
