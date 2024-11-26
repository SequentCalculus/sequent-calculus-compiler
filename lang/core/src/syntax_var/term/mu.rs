use printer::{theme::ThemeExt, tokens::DOT, DocAllocator, Print};

use super::FsTerm;
use crate::{
    syntax_var::{Chirality, FsStatement, Var},
    traits::substitution::SubstVar,
};

use std::rc::Rc;

/// Either a Mu or a TildeMu abstraction.
/// - A Mu abstraction if `chi = Prd`
/// - A TildeMu abstraction if `chi = Cns`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsMu {
    pub chi: Chirality,
    pub variable: Var,
    pub statement: Rc<FsStatement>,
}

impl FsMu {
    /// Create a new Mu abstraction
    #[allow(clippy::self_named_constructors)]
    pub fn mu<T: Into<FsStatement>>(var: &str, statement: T) -> Self {
        FsMu {
            chi: Chirality::Prd,
            variable: var.to_string(),
            statement: Rc::new(statement.into()),
        }
    }
    /// Create a new TildeMu abstraction
    pub fn tilde_mu<T: Into<FsStatement>>(var: &str, statement: T) -> Self {
        FsMu {
            chi: Chirality::Cns,
            variable: var.to_string(),
            statement: Rc::new(statement.into()),
        }
    }
}

impl Print for FsMu {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let symbol = if self.chi == Chirality::Prd {
            "mu"
        } else {
            "mutilde"
        };
        let prefix = alloc
            .keyword(symbol)
            .append(alloc.space())
            .append(self.variable.print(cfg, alloc))
            .append(DOT);
        let tail = alloc
            .line()
            .append(self.statement.print(cfg, alloc))
            .nest(cfg.indent);
        prefix.append(tail).group()
    }
}

impl From<FsMu> for FsTerm {
    fn from(value: FsMu) -> Self {
        FsTerm::Mu(value)
    }
}

impl SubstVar for FsMu {
    type Target = FsMu;
    fn subst_sim(self, subst: &[(Var, Var)]) -> FsMu {
        FsMu {
            chi: self.chi,
            variable: self.variable,
            statement: self.statement.subst_sim(subst),
        }
    }
}
