use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_ty,
};
use core::syntax::{
    term::{Cns, Prd},
    types::Ty,
};

impl CompileWithCont for fun::syntax::terms::Label {
    /// ```text
    /// 〚label a {t} 〛_{c} = ⟨μa. 〚t 〛_{a} | c⟩
    /// 〚label a {t} 〛 = μa. 〚t 〛_{a}
    /// ```
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core::syntax::term::Term<Prd> {
        let var_ty = compile_ty(
            self.ty
                .expect("Types should be annotated before translation"),
        );
        let cont = core::syntax::term::XVar {
            prdcns: Cns,
            var: self.label.clone(),
            ty,
        }
        .into();

        core::syntax::term::Mu {
            prdcns: Prd,
            variable: self.label,
            ty: var_ty,
            statement: Rc::new(self.term.compile_with_cont(cont, state)),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let ty = compile_ty(
            self.ty
                .clone()
                .expect("Types should be annotated before translation"),
        );
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

    use fun::{parse_term, typing::check::terms::Check};

    use crate::definition::CompileWithCont;
    use core::syntax::term::{Cns, Prd};
    use std::rc::Rc;

    #[test]
    fn compile_label1() {
        let term = parse_term!("label 'a { 1 }");
        let term_typed = term
            .check(
                &Default::default(),
                &vec![],
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core::syntax::types::Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            ty: core::syntax::types::Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    ty: core::syntax::types::Ty::Int(),
                    consumer: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: core::syntax::types::Ty::Int(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_label2() {
        let term = parse_term!("label 'a { goto(1;'a) }");
        let term_typed = term
            .check(
                &Default::default(),
                &vec![],
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core::syntax::types::Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            ty: core::syntax::types::Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    ty: core::syntax::types::Ty::Int(),
                    consumer: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: core::syntax::types::Ty::Int(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
