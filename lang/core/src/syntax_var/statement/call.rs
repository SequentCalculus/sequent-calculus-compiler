use printer::{DocAllocator, Print};

use crate::{
    syntax_var::{Name, Statement, Var},
    traits::substitution::SubstVar,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub name: Name,
    pub args: Vec<Var>,
}

impl Print for Call {
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

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}

impl SubstVar for Call {
    type Target = Call;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Call {
        Call {
            name: self.name,
            args: self.args.subst_sim(subst),
        }
    }
}
