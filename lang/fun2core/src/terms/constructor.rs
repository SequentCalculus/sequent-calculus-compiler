use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Constructor {
    /// ```text
    /// 〚K(t_1,...) 〛_{c} =⟨ K( 〚t_1〛,...) | c⟩
    /// 〚K(t_1,...) 〛 = K( 〚t_1〛,...)
    ///
    /// ```
    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        let new_prods = self
            .args
            .iter()
            .cloned()
            .map(|t| t.compile_opt(st))
            .collect();
        core::syntax::Constructor {
            id: self.id.compile(st),
            producers: new_prods,
            consumers: vec![],
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::Cut {
            producer: Rc::new(self.compile_opt(st)),
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;

    fn example_nil() -> fun::syntax::Constructor {
        fun::syntax::Constructor {
            id: fun::syntax::Ctor::Nil,
            args: vec![],
        }
    }
    fn example_cons() -> fun::syntax::Constructor {
        fun::syntax::Constructor {
            id: fun::syntax::Ctor::Cons,
            args: vec![
                fun::syntax::Term::Lit(1),
                fun::syntax::Constructor {
                    id: fun::syntax::Ctor::Nil,
                    args: vec![],
                }
                .into(),
            ],
        }
    }
    fn example_tup() -> fun::syntax::Constructor {
        fun::syntax::Constructor {
            id: fun::syntax::Ctor::Tup,
            args: vec![fun::syntax::Term::Lit(1), fun::syntax::Term::Lit(2)],
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
