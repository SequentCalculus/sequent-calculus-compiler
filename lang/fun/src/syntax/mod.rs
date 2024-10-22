use std::fmt;

use printer::{DocAllocator, Print};

pub type Variable = String;
pub type Covariable = String;
pub type Name = String;

pub mod context;
pub mod declarations;
pub mod substitution;
pub mod terms;
pub mod types;

// BinOp
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Prod,
    Sum,
    Sub,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Prod => write!(f, "*"),
            BinOp::Sum => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
        }
    }
}

impl Print for BinOp {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            BinOp::Prod => alloc.text("*"),
            BinOp::Sum => alloc.text("+"),
            BinOp::Sub => alloc.text("-"),
        }
    }
}
