use printer::{theme::ThemeExt, tokens::RETURN, DocAllocator, Print};

use super::{Statement, Var};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Return {
    pub var: Var,
}

impl Print for Return {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(RETURN)
            .append(alloc.space())
            .append(&self.var)
    }
}

impl From<Return> for Statement {
    fn from(value: Return) -> Self {
        Statement::Return(value)
    }
}
