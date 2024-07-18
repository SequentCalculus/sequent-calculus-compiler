use std::fmt;

type Variable = &'static str;

pub enum Term { 
    Var(Variable)
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt :: Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f,"{}",v)
        }
    }
}
