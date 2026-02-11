//! This module defines some utilities to deal with names and lists of names.
use printer::*;
use std::fmt;

/// Type of variables and covariables
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var {
    /// Identifier
    pub name: String,
    /// Index
    pub id: usize,
}

impl Var {
    pub fn fresh(name: &str, used: &[Self]) -> Self {
        let mut new_var = Var {
            name: name.to_string(),
            id: 0,
        };
        while used.contains(&new_var) {
            new_var.id += 1;
        }
        new_var
    }
}

/// Type alias for names of top-level functions, user-declared types and xtors
pub type Name = String;

/// Type alias for type variables
pub type TypeVar = String;

impl Print for Var {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name
            .print(cfg, alloc)
            .append("_")
            .append(self.id.to_string())
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.name, self.id)
    }
}
