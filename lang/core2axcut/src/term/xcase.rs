use core::syntax_var::{term::XCase, Var};

use crate::traits::UsedBinders;

use std::collections::HashSet;

impl UsedBinders for XCase {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.clauses.used_binders(used);
    }
}
