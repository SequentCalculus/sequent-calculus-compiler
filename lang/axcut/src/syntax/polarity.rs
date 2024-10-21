use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Polarity {
    Prd,
    Cns,
    Ext,
}

impl std::fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Polarity::Prd => write!(f, "prd"),
            Polarity::Cns => write!(f, "cns"),
            Polarity::Ext => write!(f, "ext"),
        }
    }
}
