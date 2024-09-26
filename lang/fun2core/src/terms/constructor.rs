use std::rc::Rc;

use crate::{
    definition::{Compile, CompileState, CompileWithCont},
    program::compile_subst,
};
use fun::syntax::substitution::subst_covars;

impl CompileWithCont for fun::syntax::terms::Constructor {
    /// ```text
    /// 〚K(t_1, ...) 〛_{c} = ⟨K( 〚t_1〛, ...) | c⟩
    /// 〚K(t_1, ...) 〛 = K( 〚t_1〛, ...)
    /// ```
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        state.covars.extend(subst_covars(&self.args));
        core::syntax::Constructor {
            id: self.id.compile(state),
            args: compile_subst(self.args, state),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::Cut {
            producer: Rc::new(self.compile_opt(state)),
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::CompileWithCont;

    #[test]
    fn compile_cons() {
        let term = parse_term!("Cons(1,Nil)");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Constructor {
            id: core::syntax::Ctor::Cons,
            args: vec![
                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                    core::syntax::Literal { lit: 1 }.into(),
                ),
                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                    core::syntax::Constructor {
                        id: core::syntax::Ctor::Nil,
                        args: vec![],
                    }
                    .into(),
                ),
            ],
        }
        .into();
        assert_eq!(result, expected)
    }
}
