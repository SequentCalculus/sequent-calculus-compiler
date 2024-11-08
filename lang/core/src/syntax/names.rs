use printer::{
    tokens::{MINUS, PLUS, TIMES},
    DocAllocator, Print,
};

pub type Var = String;
pub type Covar = String;
pub type Name = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Prod,
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
            BinOp::Prod => alloc.text(TIMES),
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
