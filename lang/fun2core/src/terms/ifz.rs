use crate::compile::{CompileState, CompileWithCont, share};
use core_lang::syntax::{Ty, terms::Cns};

use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::IfZ {
    /// ```text
    /// 〚IfZ(t_1) {t_2} else {t_3} 〛_{c} =
    ///     IfZ(〚t_1 〛, 〚t_2 〛_{μ~x.share(fv(c), x)}, 〚t_3 〛_{μ~x.share(fv(c), x)})
    /// WITH
    /// def share(fv(c), x) { < x | c > }
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        // if the consumer is a not a leaf, we share it by lifting it to the top level to avoid
        // exponential blowup
        let cont = if matches!(
                cont,
                core_lang::syntax::Term::XVar(_)
            )
            // check if consumer is μ~x.exit p with p a leaf
            || matches!(&cont, core_lang::syntax::Term::Mu(core_lang::syntax::terms::Mu { statement, .. })
                if (matches!(&**statement, core_lang::syntax::Statement::Exit(core_lang::syntax::statements::Exit { arg, .. })
                    if matches!(**arg, core_lang::syntax::Term::XVar(_)) || matches!(**arg, core_lang::syntax::Term::Literal(_))))
            ) {
            cont
        } else {
            share(cont, state)
        };

        core_lang::syntax::statements::IfZ {
            sort: match self.sort {
                fun::syntax::terms::IfZSort::Equal => core_lang::syntax::statements::IfZSort::Equal,
                fun::syntax::terms::IfZSort::NotEqual => {
                    core_lang::syntax::statements::IfZSort::NotEqual
                }
            },
            ifc: Rc::new(self.ifc.compile_opt(state, Ty::I64)),
            thenc: Rc::new(self.thenc.compile_with_cont(cont.clone(), state)),
            elsec: Rc::new(self.elsec.compile_with_cont(cont, state)),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{CompileState, CompileWithCont};
    use fun::{parse_term, typing::check::Check};

    use std::collections::{HashSet, VecDeque};

    #[test]
    fn compile_ifz1() {
        let term = parse_term!("if 0 == 0 {1} else {2}");

        let mut state = CompileState {
            used_vars: HashSet::default(),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term.compile_opt(&mut state, core_lang::syntax::types::Ty::I64);

        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::IfZ::new(
                core_lang::syntax::terms::Literal::new(0),
                core_lang::syntax::statements::Cut::new(
                    core_lang::syntax::terms::Literal::new(1),
                    core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::statements::Cut::new(
                    core_lang::syntax::terms::Literal::new(2),
                    core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_ifz2() {
        let term = parse_term!("if x == 0 {1} else {x}");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut Default::default(),
                &ctx,
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile_opt(&mut state, core_lang::syntax::types::Ty::I64);

        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::IfZ::new(
                core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::statements::Cut::new(
                    core_lang::syntax::terms::Literal::new(1),
                    core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::statements::Cut::new(
                    core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
