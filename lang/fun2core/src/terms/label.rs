use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};
use core::syntax::term::{Cns, Prd};

impl CompileWithCont for fun::syntax::terms::Label {
    /// ```text
    /// 〚label a {t} 〛_{c} = ⟨μa. 〚t 〛_{a} | c⟩
    /// 〚label a {t} 〛 = μa. 〚t 〛_{a}
    /// ```
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::term::Term<Prd> {
        let cont = core::syntax::term::XVar {
            prdcns: Cns,
            var: self.label.clone(),
        }
        .into();

        core::syntax::term::Mu {
            prdcns: Prd,
            variable: self.label,
            statement: Rc::new(self.term.compile_with_cont(cont, state)),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::statement::Cut {
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
    use core::syntax::term::{Cns, Prd};
    use std::rc::Rc;

    #[test]
    fn compile_label1() {
        let term = parse_term!("label 'a { 1 }");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
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
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
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
