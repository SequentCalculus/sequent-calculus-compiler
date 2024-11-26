use printer::{DocAllocator, Print};

use super::FsTerm;
use crate::{
    syntax_var::{Chirality, Var},
    traits::substitution::SubstVar,
};

/// Either a variable or a covariable:
/// - A variable if `T = Prd`
/// - A covariable if `T = Cns`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsXVar {
    pub chi: Chirality,
    pub var: Var,
}

impl FsXVar {
    /// Create a new variable with the given name.
    #[must_use]
    pub fn var(name: &str) -> Self {
        FsXVar {
            chi: Chirality::Prd,
            var: name.to_string(),
        }
    }
    #[must_use]
    pub fn covar(name: &str) -> Self {
        FsXVar {
            chi: Chirality::Cns,
            var: name.to_string(),
        }
    }
}

impl Print for FsXVar {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(&self.var)
    }
}

impl From<FsXVar> for FsTerm {
    fn from(value: FsXVar) -> Self {
        FsTerm::XVar(value)
    }
}

impl SubstVar for FsXVar {
    type Target = FsXVar;

    fn subst_sim(mut self, subst: &[(Var, Var)]) -> FsXVar {
        match subst.iter().find(|(old, _)| *old == self.var) {
            None => self,
            Some((_, new)) => {
                self.var = new.clone();
                self
            }
        }
    }
}
