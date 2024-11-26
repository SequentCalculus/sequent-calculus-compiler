use printer::{DocAllocator, Print};

use super::FsTerm;
use crate::{
    syntax_var::{Name, Var},
    traits::substitution::SubstVar,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsXtor {
    pub id: Name,
    pub args: Vec<Var>,
}

impl Print for FsXtor {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        alloc.text(&self.id).append(args)
    }
}

impl From<FsXtor> for FsTerm {
    fn from(value: FsXtor) -> Self {
        FsTerm::Xtor(value)
    }
}

impl SubstVar for FsXtor {
    type Target = FsXtor;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        FsXtor {
            id: self.id,
            args: self.args.subst_sim(subst),
        }
    }
}
