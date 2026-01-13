//! This module defines some utilities to deal with names and lists of names.
use printer::*;
use std::fmt;

/// Type for (Co-) Variables
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var {
    /// Name of the variable
    pub name: Name,
    /// numeric id to avoid renaming
    pub id: usize,
}

/// Type alias for names of top-level functions, user-declared types and xtors
pub type Name = String;

impl Print for Var {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name.print(cfg, alloc)
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
