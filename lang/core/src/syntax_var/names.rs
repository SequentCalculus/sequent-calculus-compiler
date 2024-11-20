use printer::{
    tokens::{DIVIDE, MINUS, MODULO, PLUS, TIMES},
    DocAllocator, Print,
};

use crate::traits::substitution::SubstVar;

pub type Var = String;
pub type Name = String;

impl SubstVar for Var {
    type Target = Var;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Var {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Div,
    Prod,
    Rem,
    Sum,
    Sub,
}

impl Print for BinOp {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            BinOp::Div => alloc.text(DIVIDE),
            BinOp::Prod => alloc.text(TIMES),
            BinOp::Rem => alloc.text(MODULO),
            BinOp::Sum => alloc.text(PLUS),
            BinOp::Sub => alloc.text(MINUS),
        }
    }
}
