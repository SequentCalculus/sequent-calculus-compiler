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
    use crate::definition::CompileWithCont;

    fn example_nil() -> fun::syntax::terms::Constructor {
        fun::syntax::terms::Constructor {
            id: fun::syntax::Ctor::Nil,
            args: vec![],
        }
    }

    fn example_cons() -> fun::syntax::terms::Constructor {
        fun::syntax::terms::Constructor {
            id: fun::syntax::Ctor::Cons,
            args: vec![
                fun::syntax::terms::Lit { val: 1 }.into(),
                fun::syntax::terms::Constructor {
                    id: fun::syntax::Ctor::Nil,
                    args: vec![],
                }
                .into(),
            ],
        }
    }

    fn example_tup() -> fun::syntax::terms::Constructor {
        fun::syntax::terms::Constructor {
            id: fun::syntax::Ctor::Tup,
            args: vec![
                fun::syntax::terms::Lit { val: 1 }.into(),
                fun::syntax::terms::Lit { val: 2 }.into(),
            ],
        }
    }

    #[test]
    fn compile_nil() {
        let result = example_nil().compile_opt(&mut Default::default());
        let expected = core::syntax::Constructor {
            id: core::syntax::Ctor::Nil,
            producers: vec![],
            consumers: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_cons() {
        let result = example_cons().compile_opt(&mut Default::default());
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

    #[test]
    fn compile_tup() {
        let result = example_tup().compile_opt(&mut Default::default());
        let expected = core::syntax::Constructor {
            id: core::syntax::Ctor::Tup,
            producers: vec![
                core::syntax::Literal { lit: 1 }.into(),
                core::syntax::Literal { lit: 2 }.into(),
            ],
            consumers: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }
}
