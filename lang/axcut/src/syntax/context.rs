use super::{names::Var, polarity::Polarity, types::Ty};

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct ContextBinding {
    pub var: Var,
    pub pol: Polarity,
    pub ty: Ty,
}

pub type TypingContext = Vec<ContextBinding>;

impl fmt::Display for ContextBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} :{}: {}", self.var, self.pol, self.ty)
    }
}
