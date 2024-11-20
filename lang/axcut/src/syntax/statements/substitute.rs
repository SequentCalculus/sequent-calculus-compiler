use printer::theme::ThemeExt;
use printer::tokens::{COMMA, SEMI, SUBSTITUTE};
use printer::{DocAllocator, Print};

use super::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::UsedBinders;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Substitute {
    pub rearrange: Vec<(Var, Var)>,
    pub next: Rc<Statement>,
}

impl Print for Substitute {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let rearrange = alloc.intersperse(
            self.rearrange
                .iter()
                .map(|(new, old)| alloc.text(new).append(" !-> ").append(old).parens()),
            alloc.text(COMMA).append(alloc.space()),
        );
        alloc
            .keyword(SUBSTITUTE)
            .append(alloc.space())
            .append(rearrange)
            .append(SEMI)
            .append(alloc.space())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<Substitute> for Statement {
    fn from(value: Substitute) -> Self {
        Statement::Substitute(value)
    }
}

impl FreeVars for Substitute {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.next.free_vars(vars);
        for (new, old) in &self.rearrange {
            vars.insert(old.clone());
            vars.remove(new);
        }
    }
}

impl Subst for Substitute {
    type Target = Substitute;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Substitute {
        Substitute {
            rearrange: self
                .rearrange
                .into_iter()
                .map(|(new, old)| (new, old.subst_sim(subst)))
                .collect(),
            next: self.next.subst_sim(subst),
        }
    }
}

impl UsedBinders for Substitute {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for (new, _) in &self.rearrange {
            used.insert(new.clone());
        }
        self.next.used_binders(used);
    }
}

#[cfg(test)]
mod substitute_tests {
    use super::{FreeVars, Subst, Substitute, UsedBinders};
    use crate::syntax::statements::Return;
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_subst() -> Substitute {
        Substitute {
            rearrange: vec![
                ("x".to_owned(), "y".to_owned()),
                ("a".to_owned(), "b".to_owned()),
            ],
            next: Rc::new(
                Return {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn print_subst() {
        let result = example_subst().print_to_string(Default::default());
        let expected = "substitute (x !-> y), (a !-> b); return x";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_subst() {
        let mut result = HashSet::new();
        example_subst().free_vars(&mut result);
        let expected = HashSet::from(["b".to_owned(), "y".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_subst() {
        let result = example_subst().subst_sim(&vec![
            ("x".to_owned(), "z".to_owned()),
            ("a".to_owned(), "c".to_owned()),
        ]);
        let expected = Substitute {
            rearrange: vec![
                ("x".to_owned(), "y".to_owned()),
                ("a".to_owned(), "b".to_owned()),
            ],
            next: Rc::new(
                Return {
                    var: "z".to_owned(),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn used_binders_subst() {
        let mut result = HashSet::new();
        example_subst().used_binders(&mut result);
        let expected = HashSet::from(["x".to_owned(), "a".to_owned()]);
        assert_eq!(result, expected)
    }
}
