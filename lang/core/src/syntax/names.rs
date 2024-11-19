use printer::{
    tokens::{DIVIDE, MINUS, MODULO, PLUS, TIMES},
    DocAllocator, Print,
};

use crate::traits::focus::{Focusing, FocusingState};

pub type Var = String;
pub type Covar = String;
pub type Name = String;

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

#[cfg(test)]
mod names_tests {
    use printer::Print;

    use super::BinOp;

    #[test]
    fn display_prod() {
        let result = BinOp::Prod.print_to_string(None);
        let expected = "*".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_sum() {
        let result = BinOp::Sum.print_to_string(None);
        let expected = "+".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_sub() {
        let result = BinOp::Sub.print_to_string(None);
        let expected = "-".to_owned();
        assert_eq!(result, expected)
    }
}

impl Focusing for BinOp {
    type Target = crate::syntax_var::BinOp;
    fn focus(self, _state: &mut FocusingState) -> crate::syntax_var::BinOp {
        match self {
            BinOp::Div => crate::syntax_var::BinOp::Div,
            BinOp::Prod => crate::syntax_var::BinOp::Prod,
            BinOp::Rem => crate::syntax_var::BinOp::Rem,
            BinOp::Sum => crate::syntax_var::BinOp::Sum,
            BinOp::Sub => crate::syntax_var::BinOp::Sub,
        }
    }
}
