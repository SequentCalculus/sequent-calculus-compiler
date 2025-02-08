use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_subst, compile_ty},
};
use core_lang::syntax::{
    term::{Cns, Prd},
    Ty,
};

impl CompileWithCont for fun::syntax::terms::Constructor {
    /// ```text
    /// 〚K(t_1, ...) 〛 = K( 〚t_1〛, ...)
    /// ```
    fn compile_opt(self, state: &mut CompileState, _ty: Ty) -> core_lang::syntax::term::Term<Prd> {
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

    /// ```text
    /// 〚K(t_1, ...) 〛_{c} = ⟨K( 〚t_1〛, ...) | c⟩
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
    use fun::{
        parse_term, syntax::context::TypingContext, test_common::symbol_table_list,
        typing::check::Check,
    };

    use crate::definition::CompileWithCont;

    #[test]
    fn compile_cons() {
        let term = parse_term!("Cons(1,Nil)");
        let term_typed = term
            .check(
                &mut symbol_table_list(),
                &TypingContext::default(),
                &fun::syntax::types::Ty::mk_decl(
                    "List",
                    fun::syntax::types::TypeArgs::mk(vec![fun::syntax::types::Ty::mk_i64()]),
                ),
            )
            .unwrap();
        let result = term_typed.compile_opt(
            &mut Default::default(),
            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
        );
        let mut subst = core_lang::syntax::substitution::Substitution::default();
        subst.add_prod(core_lang::syntax::term::Literal::new(1));
        subst.add_prod(core_lang::syntax::term::Xtor::ctor(
            "Nil",
            core_lang::syntax::substitution::Substitution::default(),
            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
        ));
        let expected = core_lang::syntax::term::Xtor::ctor(
            "Cons",
            subst,
            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
        )
        .into();
        assert_eq!(result, expected)
    }
}
