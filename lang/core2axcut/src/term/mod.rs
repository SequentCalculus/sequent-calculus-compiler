use core::syntax_var::Var;

use core::syntax_var::Term;

use crate::traits::UsedBinders;

use std::collections::HashSet;

pub mod mu;
pub mod xcase;

impl UsedBinders for Term {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Term::Mu(m) => m.used_binders(used),
            Term::XCase(c) => c.used_binders(used),
            _ => {}
        }
    }
}
