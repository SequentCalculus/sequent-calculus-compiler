use std::fmt;

pub type Var = String;
pub type Covar = String;
pub type Name = String;

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

#[cfg(test)]
mod names_tests {
    use super::BinOp;

    #[test]
    fn display_prod() {
        let result = format!("{}", BinOp::Prod);
        let expected = "*".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_sum() {
        let result = format!("{}", BinOp::Sum);
        let expected = "+".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_sub() {
        let result = format!("{}", BinOp::Sub);
        let expected = "-".to_owned();
        assert_eq!(result, expected)
    }
}
