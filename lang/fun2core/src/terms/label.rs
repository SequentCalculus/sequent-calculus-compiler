//! This module defines the translation for the goto control operator.

use crate::{
    compile::{Compile, CompileState},
    types::compile_ty,
};
use core_lang::syntax::{
    Ty,
    terms::{Cns, Prd},
};

use std::rc::Rc;

impl Compile for fun::syntax::terms::Label {
    /// This implementation of [Compile::compile] proceeds as follows.
    /// ```text
    /// 〚label a {t} 〛 = μa. 〚t 〛_{a}
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile(self, state: &mut CompileState, _ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        let var_ty = compile_ty(
            &self
                .ty
                .expect("Types should be annotated before translation"),
        );
        let cont = core_lang::syntax::terms::XVar {
            prdcns: Cns,
            var: self.label.clone(),
            ty: var_ty.clone(),
        }
        .into();

        core_lang::syntax::terms::Mu {
            prdcns: Prd,
            variable: self.label,
            ty: var_ty,
            statement: Rc::new(self.term.compile_with_cont(cont, state)),
        }
        .into()
    }

    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚label a {t} 〛_{c} = ⟨μa. 〚t 〛_{a} | c⟩
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(
            &self
                .ty
                .clone()
                .expect("Types should be annotated before translation"),
        );
        core_lang::syntax::statements::Cut {
            producer: Rc::new(self.compile(state, ty.clone())),
            ty,
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use fun::{parse_term, typing::check::Check};
    use macros::{covar, cut, lit, mu, ty};
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn compile_label1() {
        let term = parse_term!("label a { 1 }");
        let term_typed = term
            .check(
                &mut Default::default(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from(["a".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("int"));

        let expected = mu!("a", cut!(lit!(1), covar!("a"))).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_label2() {
        let term = parse_term!("label a { goto a (1) }");
        let term_typed = term
            .check(
                &mut Default::default(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from(["a".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("int"));

        let expected = mu!("a", cut!(lit!(1), covar!("a"))).into();
        assert_eq!(result, expected)
    }
}
