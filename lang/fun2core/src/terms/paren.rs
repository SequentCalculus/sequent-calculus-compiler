use crate::definition::{CompileState, CompileWithCont};
use core::syntax::{
    term::{Cns, Prd},
    types::Ty,
};

impl CompileWithCont for fun::syntax::terms::Paren {
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core::syntax::term::Term<Prd> {
        self.inner.compile_opt(state, ty)
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
    use crate::definition::CompileWithCont;
    use core::syntax::types::Ty;
    use fun::{parse_term, typing::check::terms::Check};
    use std::rc::Rc;

    #[test]
    fn compile_paren1() {
        let term = parse_term!("(1)");
        let result = term.compile_opt(&mut Default::default(), Ty::Int());
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
                ty: core::syntax::types::Ty::Int(),
            }
            .into(),
            &mut Default::default(),
        );
        let expected = core::syntax::statement::Cut {
            producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
            ty: core::syntax::types::Ty::Int(),
            consumer: Rc::new(
                core::syntax::term::XVar {
                    prdcns: core::syntax::term::Cns,
                    var: "a".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
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
        let term_typed = term
            .check(
                &Default::default(),
                &vec![fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                }],
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result = term_typed.compile_opt(&mut Default::default(), Ty::Int());
        let expected = core::syntax::term::XVar {
            prdcns: core::syntax::term::Prd,
            var: "x".to_owned(),
            ty: core::syntax::types::Ty::Int(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_inner_paren2() {
        let term = parse_term!("(x)");
        let term_typed = term
            .check(
                &Default::default(),
                &vec![fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                }],
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result = term_typed.compile_with_cont(
            core::syntax::term::XVar {
                prdcns: core::syntax::term::Cns,
                var: "a".to_owned(),
                ty: core::syntax::types::Ty::Int(),
            }
            .into(),
            &mut Default::default(),
        );
        let expected = core::syntax::statement::Cut {
            producer: Rc::new(
                core::syntax::term::XVar {
                    prdcns: core::syntax::term::Prd,
                    var: "x".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                }
                .into(),
            ),
            ty: core::syntax::types::Ty::Int(),
            consumer: Rc::new(
                core::syntax::term::XVar {
                    prdcns: core::syntax::term::Cns,
                    var: "a".to_owned(),
                    ty: core::syntax::types::Ty::Int(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
