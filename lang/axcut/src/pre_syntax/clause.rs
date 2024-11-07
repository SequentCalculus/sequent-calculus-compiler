use super::statement::Statement;
use crate::syntax::{stringify_and_join, Name, TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::UsedBinders;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub case: Rc<Statement>,
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context = stringify_and_join(&self.context, ", ");
        write!(f, "{}({context}) =>\n  {}", self.xtor, self.case)
    }
}

impl FreeVars for Clause {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        for binding in &self.context {
            vars.remove(&binding.var);
        }
    }
}

impl Subst for Clause {
    type Target = Clause;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Clause {
        Clause {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for Clause {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for binding in &self.context {
            used.insert(binding.var.clone());
        }
        self.case.used_binders(used);
    }
}
