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
    type Target = Substitute;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Substitute {
        let mut free_vars = HashSet::new();
        self.case.free_vars(&mut free_vars);

        let mut new_context = filter_by_set(&context, &free_vars);

        let rearrange = new_context
            .clone()
            .into_iter()
            .zip(new_context.clone())
            .collect();

        new_context.push(self.var.clone());

        Substitute {
            rearrange,
            next: Rc::new(
                Literal {
                    lit: self.lit,
                    var: self.var,
                    case: self.case.linearize(new_context, used_vars),
                }
                .into(),
            ),
        }
    }
}
