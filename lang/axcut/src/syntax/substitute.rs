use printer::Print;

use super::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::UsedBinders;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Substitute {
    pub rearrange: Vec<(Var, Var)>,
    pub next: Rc<Statement>,
}

impl std::fmt::Display for Substitute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rearrange = if self.rearrange.is_empty() {
            "()".to_string()
        } else {
            self.rearrange
                .iter()
                .map(|(new, old)| format!("({new} !-> {old})"))
                .collect::<Vec<String>>()
                .join(" ")
        };
        write!(f, "substitute {};\n  {}", rearrange, self.next)
    }
}

impl Print for Substitute {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        todo!()
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
