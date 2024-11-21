use printer::theme::ThemeExt;
use printer::tokens::{LEFT_ARROW, LIT, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::filter_by_set, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
    pub var: Var,
    pub case: Rc<Statement>,
}

impl Print for Literal {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(LIT)
            .append(alloc.space())
            .append(&self.var)
            .append(alloc.space())
            .append(LEFT_ARROW)
            .append(alloc.space())
            .append(format!("{}", self.lit))
            .append(SEMI)
            .append(alloc.space())
            .append(self.case.print(cfg, alloc))
    }
}
impl From<Literal> for Statement {
    fn from(value: Literal) -> Self {
        Statement::Literal(value)
    }
}

impl FreeVars for Literal {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.remove(&self.var);
    }
}

impl Subst for Literal {
    type Target = Literal;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Literal {
        Literal {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for Literal {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.var.clone());
        self.case.used_binders(used);
    }
}

impl Linearizing for Literal {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.case.free_vars(&mut free_vars);

        let mut new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        new_context.push(self.var.clone());
        let literal = Literal {
            lit: self.lit,
            var: self.var,
            case: self.case.linearize(new_context, used_vars),
        }
        .into();

        if context == context_rearrange {
            literal
        } else {
            let rearrange = context_rearrange
                .clone()
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(literal),
            }
            .into()
        }
    }
}

#[cfg(test)]
mod literal_tests {
    use super::{FreeVars, Linearizing, Literal, Subst, UsedBinders};
    use crate::syntax::statements::Return;
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_lit() -> Literal {
        Literal {
            lit: 1,
            var: "x".to_owned(),
            case: Rc::new(
                Return {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn print_lit() {
        let result = example_lit().print_to_string(Default::default());
        let expected = "lit x <- 1; return x";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_lit() {
        let mut result = HashSet::new();
        example_lit().free_vars(&mut result);
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_lit() {
        let result = example_lit().subst_sim(&vec![("x".to_owned(), "y".to_owned())]);
        let expected = Literal {
            var: "x".to_owned(),
            lit: 1,
            case: Rc::new(
                Return {
                    var: "y".to_owned(),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn used_binders_lit() {
        let mut result = HashSet::new();
        example_lit().used_binders(&mut result);
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_lit() {
        let result = example_lit().linearize(vec![], &mut HashSet::new());
        let expected = Literal {
            lit: 1,
            var: "x".to_owned(),
            case: Rc::new(
                Return {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
