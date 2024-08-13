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
