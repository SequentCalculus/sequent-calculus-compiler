use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Chirality {
    Prd,
    Cns,
}

impl std::fmt::Display for Chirality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chirality::Prd => write!(f, "prd"),
            Chirality::Cns => write!(f, "cns"),
        }
    }
}
