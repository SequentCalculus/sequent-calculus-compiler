use std::fmt;

pub type Variable = String;
pub type Covariable = String;
pub type Name = String;

pub mod context;
pub mod declarations;
pub mod kinds;
pub mod terms;
pub mod types;

fn stringify_and_join<T: fmt::Display>(vec: &[T]) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

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

// Ctor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ctor {
    Nil,
    Cons,
    Tup,
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

// Dtor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dtor {
    Hd,
    Tl,
    Fst,
    Snd,
    Ap,
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
