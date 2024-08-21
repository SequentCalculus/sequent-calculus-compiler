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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ctor {
    Nil,
    Cons,
    Tup,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dtor {
    Hd,
    Tl,
    Fst,
    Snd,
    Ap,
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

impl fmt::Display for Ctor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ctor::Nil => write!(f, "Nil"),
            Ctor::Cons => write!(f, "Cons"),
            Ctor::Tup => write!(f, "Tup"),
        }
    }
}

impl fmt::Display for Dtor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dtor::Hd => write!(f, "hd"),
            Dtor::Tl => write!(f, "tl"),
            Dtor::Fst => write!(f, "fst"),
            Dtor::Snd => write!(f, "snd"),
            Dtor::Ap => write!(f, "ap"),
        }
    }
}

#[cfg(test)]
mod names_tests {
    use super::{BinOp, Ctor, Dtor};

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

    #[test]
    fn display_nil() {
        let result = format!("{}", Ctor::Nil);
        let expected = "Nil".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_cons() {
        let result = format!("{}", Ctor::Cons);
        let expected = "Cons".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_tup() {
        let result = format!("{}", Ctor::Tup);
        let expected = "Tup".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_hd() {
        let result = format!("{}", Dtor::Hd);
        let expected = "hd".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_tl() {
        let result = format!("{}", Dtor::Tl);
        let expected = "tl".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_fst() {
        let result = format!("{}", Dtor::Fst);
        let expected = "fst".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_snd() {
        let result = format!("{}", Dtor::Snd);
        let expected = "snd".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_ap() {
        let result = format!("{}", Dtor::Ap);
        let expected = "ap".to_owned();
        assert_eq!(result, expected)
    }
}
