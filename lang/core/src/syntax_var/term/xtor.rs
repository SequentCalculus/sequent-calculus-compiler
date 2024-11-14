use printer::{DocAllocator, Print};

use super::Term;
use crate::{
    syntax_var::{Name, Var},
    traits::substitution::SubstVar,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor {
    pub id: Name,
    pub args: Vec<Var>,
}

impl Print for Xtor {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.id)
            .append(self.args.print(cfg, alloc).parens())
    }
}

impl From<Xtor> for Term {
    fn from(value: Xtor) -> Self {
        Term::Xtor(value)
    }
}

impl SubstVar for Xtor {
    type Target = Xtor;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        Xtor {
            id: self.id,
            args: self.args.subst_sim(subst),
        }
    }
}
