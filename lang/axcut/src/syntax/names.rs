//! This module defines some utilities to deal with names and lists of names.

use crate::traits::substitution::Subst;
use printer::*;

/// Type alias for unique IDs in the program.
pub type ID = usize;

/// `Identifier`s in the program, used for variables, top-level labels, and names of user-declared
/// types and their xtors. Each of the three categories lives in a separate namespace. `id`s of
/// variables are expected to be globally unique. Internally, only the `id` matters, the `name` is
/// just for pretty-printing. By convention, if the `id` is `0`, the `Identifier` is not unique.
/// For top-level labels and types and their xtors, the `id` is currently not used, requiring the
/// `name` to be unique.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier {
    /// base name of the ident
    pub name: String,
    /// unique id
    pub id: ID,
}

pub fn fresh_identifier(max_id: &mut ID, base_name: &str) -> Identifier {
    let identifier = Identifier {
        name: base_name.to_string(),
        id: *max_id + 1,
    };
    *max_id += 1;
    identifier
}

impl Subst for Identifier {
    fn subst_sim(self, subst: &[(Identifier, Identifier)]) -> Identifier {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}

impl Print for Identifier {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
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
