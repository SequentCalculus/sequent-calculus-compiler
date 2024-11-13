use super::Term;
use crate::{
    syntax_var::{Chirality, Statement, Var},
    traits::{shrink::UsedBinders, substitution::SubstVar},
};
use std::{collections::HashSet, fmt, rc::Rc};

/// Either a Mu or a TildeMu abstraction.
/// - A Mu abstraction if `chi = Prd`
/// - A TildeMu abstraction if `chi = Cns`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu {
    pub chi: Chirality,
    pub variable: Var,
    pub statement: Rc<Statement>,
}

impl Mu {
    /// Create a new Mu abstraction
    #[allow(clippy::self_named_constructors)]
    pub fn mu<T: Into<Statement>>(var: &str, statement: T) -> Self {
        Mu {
            chi: Chirality::Prd,
            variable: var.to_string(),
            statement: Rc::new(statement.into()),
        }
    }
    /// Create a new TildeMu abstraction
    pub fn tilde_mu<T: Into<Statement>>(var: &str, statement: T) -> Self {
        Mu {
            chi: Chirality::Cns,
            variable: var.to_string(),
            statement: Rc::new(statement.into()),
        }
    }
}

impl std::fmt::Display for Mu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.chi == Chirality::Prd {
            format!("mu {}", self.variable)
        } else {
            format!("mutilde {}", self.variable)
        };
        write!(f, "{}. {}", prefix, self.statement)
    }
}

impl From<Mu> for Term {
    fn from(value: Mu) -> Self {
        Term::Mu(value)
    }
}

impl SubstVar for Mu {
    type Target = Mu;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Mu {
        Mu {
            chi: self.chi,
            variable: self.variable,
            statement: self.statement.subst_sim(subst),
        }
    }
}

impl UsedBinders for Mu {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.variable.clone());
        self.statement.used_binders(used);
    }
}
