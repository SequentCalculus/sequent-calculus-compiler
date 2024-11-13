use core::syntax_var::{term::Mu, Var};

use crate::traits::UsedBinders;

use std::collections::HashSet;

impl UsedBinders for Mu {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.variable.clone());
        self.statement.used_binders(used);
    }
}
