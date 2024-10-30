use crate::definition::{CompileState, CompileWithCont};
use core::syntax::term::{Cns, Prd};

impl CompileWithCont for fun::syntax::terms::Paren {
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::term::Term<Prd> {
        self.inner.compile_opt(state)
    }

    fn compile_with_cont(
        self,
        c: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        self.inner.compile_with_cont(c, state)
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::{CompileState, CompileWithCont};
    use core::syntax::types::Ty;
    use fun::parse_term;
    use std::rc::Rc;

    #[test]
    fn compile_paren1() {
        let term = parse_term!("(1)");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Literal { lit: 1 }.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_inner_paren1() {
        let term = parse_term!("(1)");
        let result = term.compile_with_cont(
            core::syntax::term::XVar {
                prdcns: core::syntax::term::Cns,
                var: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        let expected = core::syntax::statement::Cut {
            producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
            ty: Ty::Int(),
            consumer: Rc::new(
                core::syntax::term::XVar {
                    prdcns: core::syntax::term::Cns,
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_paren2() {
        let term = parse_term!("(x)");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::XVar {
            prdcns: core::syntax::term::Prd,
            var: "x".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_inner_paren2() {
        let term = parse_term!("(x)");
        let mut st = CompileState::default();
        let result = term.compile_with_cont(
            core::syntax::term::XVar {
                prdcns: core::syntax::term::Cns,
                var: "a".to_owned(),
            }
            .into(),
            &mut st,
        );
        let expected = core::syntax::statement::Cut {
            producer: Rc::new(
                core::syntax::term::XVar {
                    prdcns: core::syntax::term::Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            ty: Ty::Int(),
            consumer: Rc::new(
                core::syntax::term::XVar {
                    prdcns: core::syntax::term::Cns,
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
