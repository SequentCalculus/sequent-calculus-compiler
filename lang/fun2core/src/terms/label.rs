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
    fn compile_opt(self, state: &mut CompileState, _: Ty) -> core::syntax::term::Term<Prd> {
        let ty_comp = compile_ty(self.ty.unwrap());
        let cont = core::syntax::term::XVar {
            prdcns: Cns,
            var: self.label.clone(),
            ty: ty_comp.clone(),
        }
        .into();
        state.covars.insert(self.label.clone(), ty_comp.clone());
        let term_comp = self.term.compile_with_cont(cont, state);

        core::syntax::term::Mu {
            prdcns: Prd,
            variable: self.label,
            var_ty: ty_comp,
            statement: Rc::new(term_comp),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::statement::Cut {
            //TODO fix type
            producer: Rc::new(self.compile_opt(state, Ty::Int())),
            ty: Ty::Int(),
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {

    use fun::{
        parse_term,
        typing::{check::terms::Check, symbol_table::SymbolTable},
    };

    use crate::definition::CompileWithCont;
    use core::syntax::{
        term::{Cns, Prd},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn compile_label1() {
        let term = parse_term!("label 'a { 1 }");
        let term_typed = term
            .check(
                &SymbolTable::default(),
                &vec![],
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result = term_typed.compile_opt(&mut Default::default(), Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: Ty::Int(),
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
                &SymbolTable::default(),
                &vec![],
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result = term_typed.compile_opt(&mut Default::default(), Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: Ty::Int(),
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
