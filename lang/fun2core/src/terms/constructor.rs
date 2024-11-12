use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_subst, compile_ty},
};
use core::syntax::{
    term::{Cns, Prd},
    types::Ty,
};
use fun::syntax::substitution::subst_covars;

impl CompileWithCont for fun::syntax::terms::Constructor {
    /// ```text
    /// 〚K(t_1, ...) 〛_{c} = ⟨K( 〚t_1〛, ...) | c⟩
    /// 〚K(t_1, ...) 〛 = K( 〚t_1〛, ...)
    /// ```
    fn compile_opt(self, state: &mut CompileState, _ty: Ty) -> core::syntax::term::Term<Prd> {
        state.covars.extend(subst_covars(&self.args));
        core::syntax::term::Xtor {
            prdcns: Prd,
            id: self.id,
            args: compile_subst(self.args, state),
            ty: compile_ty(self.ty.unwrap()),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let ty = compile_ty(self.ty.clone().unwrap());
        core::syntax::statement::Cut {
            producer: Rc::new(self.compile_opt(state, ty.clone())),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use core::syntax::term::Prd;

    #[test]
    fn compile_cons() {
        let term = parse_term!("Cons(1,Nil)");
        let result = term.compile_opt(
            &mut Default::default(),
            core::syntax::types::Ty::Decl("ListInt".to_owned()),
        );
        let expected = core::syntax::term::Xtor {
            prdcns: Prd,
            id: "Cons".to_owned(),
            args: vec![
                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                    core::syntax::term::Literal { lit: 1 }.into(),
                ),
                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                    core::syntax::term::Xtor {
                        prdcns: Prd,
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                    }
                    .into(),
                ),
            ],
            ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
        }
        .into();
        assert_eq!(result, expected)
    }
}
