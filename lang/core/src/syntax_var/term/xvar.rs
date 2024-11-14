use printer::{DocAllocator, Print};

use super::Term;
use crate::{
    syntax_var::{Chirality, Var},
    traits::substitution::SubstVar,
};

/// Either a variable or a covariable:
/// - A variable if `T = Prd`
/// - A covariable if `T = Cns`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XVar {
    pub chi: Chirality,
    pub var: Var,
}

impl XVar {
    /// Create a new variable with the given name.
    #[must_use]
    pub fn var(name: &str) -> Self {
        XVar {
            chi: Chirality::Prd,
            var: name.to_string(),
        }
    }
    #[must_use]
    pub fn covar(name: &str) -> Self {
        XVar {
            chi: Chirality::Cns,
            var: name.to_string(),
        }
    }
}

impl Print for XVar {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(&self.var)
    }
}

impl From<XVar> for Term {
    fn from(value: XVar) -> Self {
        Term::XVar(value)
    }
}

impl SubstVar for XVar {
    type Target = XVar;

    fn subst_sim(mut self, subst: &[(Var, Var)]) -> XVar {
        match subst.iter().find(|(old, _)| *old == self.var) {
            None => self,
            Some((_, new)) => {
                self.var = new.clone();
                self
            }
        }
    }
}
