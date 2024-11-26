use printer::{DocAllocator, Print};

use crate::{
    syntax_var::{FsStatement, Name, Var},
    traits::substitution::SubstVar,
};

/// Focused Call
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsCall {
    pub name: Name,
    pub args: Vec<Var>,
}

impl Print for FsCall {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.name)
            .append(self.args.print(cfg, alloc).parens())
    }
}

impl From<FsCall> for FsStatement {
    fn from(value: FsCall) -> Self {
        FsStatement::Call(value)
    }
}

impl SubstVar for FsCall {
    type Target = FsCall;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsCall {
        FsCall {
            name: self.name,
            args: self.args.subst_sim(subst),
        }
    }
}
