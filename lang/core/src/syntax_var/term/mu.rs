use printer::{theme::ThemeExt, tokens::DOT, DocAllocator, Print};

use super::Term;
use crate::{
    syntax_var::{Chirality, Statement, Var},
    traits::{substitution::SubstVar, used_binders::UsedBinders},
};

use std::collections::HashSet;
use std::rc::Rc;

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

impl Print for Mu {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let prefix = if self.chi == Chirality::Prd {
            "mu"
        } else {
            "mutilde"
        };
        alloc
            .keyword(prefix)
            .append(alloc.space())
            .append(self.variable.print(cfg, alloc))
            .append(DOT)
            .append(alloc.space())
            .append(self.statement.print(cfg, alloc))
    }
}

impl From<Mu> for Term {
    fn from(value: Mu) -> Self {
        Term::Mu(value)
    }
}

impl UsedBinders for Mu {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.variable.clone());
        self.statement.used_binders(used);
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
