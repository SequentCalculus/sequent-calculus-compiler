//! This module defines some utilities to deal with names and lists of names.

use crate::traits::*;
use printer::*;

/// Type alias for unique IDs in the program.
pub type ID = usize;

/// `Identifier`s in the program, used for (co)variables, top-level labels, and names of
/// user-declared types and their xtors. Each of the three categories lives in a separate
/// namespace. `id`s of (co)variables are made globally unique by the [`Uniquify`] pass. Thus,
/// after this pass, only the `id` matters internally, and the `name` is just for pretty-printing.
/// By convention, if the `id` is `0`, the `Identifier` is not (yet) unique. For top-level labels
/// and types and their  xtors, the `id` is currently not used, requiring the `name` to be unique.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier {
    /// base name for pretty-printing
    pub name: String,
    /// unique id
    pub id: ID,
}

impl Identifier {
    /// Create a new [`Identifier`] with id `0`.
    pub fn new(name: String) -> Self {
        Self { name, id: 0 }
    }
}

/// Create a fresh [`Identifier`] with given `base_name`. `max_id` is the current maximal ID used
/// in the program and is incremented by this function.
pub fn fresh_identifier(max_id: &mut ID, base_name: &str) -> Identifier {
    let identifier = Identifier {
        name: base_name.to_string(),
        id: *max_id + 1,
    };
    *max_id += 1;
    identifier
}

/// Create a fresh variable, i.e., a fresh [`Identifier`] with base name `x`. `max_id` is the
/// current maximal ID used in the program and is incremented by this function.
pub fn fresh_var(max_id: &mut ID) -> Identifier {
    fresh_identifier(max_id, "x")
}

/// Create a fresh covariable, i.e., a fresh [`Identifier`] with base name `a`. `max_id` is the
/// current maximal ID used in the program and is incremented by this function.
pub fn fresh_covar(max_id: &mut ID) -> Identifier {
    fresh_identifier(max_id, "a")
}

impl SubstVar for Identifier {
    type Target = Identifier;
    fn subst_sim(self, subst: &[(Identifier, Identifier)]) -> Identifier {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}

impl Print for Identifier {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        if self.id == 0 {
            self.name.print(cfg, alloc)
        } else {
            self.name
                .print(cfg, alloc)
                .append(tokens::UNDERSCORE)
                .append(self.id.to_string())
        }
    }
}
