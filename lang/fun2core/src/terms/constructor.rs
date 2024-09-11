use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};
use core::syntax::Covariable;
use fun::syntax::substitution::split_subst;

impl CompileWithCont for fun::syntax::terms::Constructor {
    /// ```text
    /// 〚K(t_1, ...) 〛_{c} = ⟨K( 〚t_1〛, ...) | c⟩
    /// 〚K(t_1, ...) 〛 = K( 〚t_1〛, ...)
    /// ```
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        let (pargs, cargs) = split_subst(self.args);
        state.covars.extend(cargs.clone());
        core::syntax::Constructor {
            id: self.id.compile(state),
            producers: pargs.into_iter().map(|p| p.compile_opt(state)).collect(),
            consumers: cargs
                .into_iter()
                .map(|cv| Covariable { covar: cv }.into())
                .collect(),
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
            producers: vec![
                core::syntax::Literal { lit: 1 }.into(),
                core::syntax::Constructor {
                    id: core::syntax::Ctor::Nil,
                    producers: vec![],
                    consumers: vec![],
                }
                .into(),
            ],
            consumers: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }
}
