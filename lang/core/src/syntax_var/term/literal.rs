use super::Term;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
}

impl Literal {
    #[must_use]
    pub fn new(lit: i64) -> Self {
        Literal { lit }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lit)
    }
}

impl From<Literal> for Term {
    fn from(value: Literal) -> Self {
        Term::Literal(value)
    }
}
