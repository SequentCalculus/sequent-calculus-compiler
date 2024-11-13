use super::Term;
use crate::{
    syntax_var::{stringify_and_join, Clause, Var},
    traits::{shrink::UsedBinders, substitution::SubstVar},
};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCase {
    pub clauses: Vec<Clause>,
}

impl std::fmt::Display for XCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses_joined = stringify_and_join(&self.clauses, ", ");
        write!(f, "case {{ {clauses_joined} }}")
    }
}

impl From<XCase> for Term {
    fn from(value: XCase) -> Self {
        Term::XCase(value)
    }
}

impl SubstVar for XCase {
    type Target = XCase;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        XCase {
            clauses: self.clauses.subst_sim(subst),
        }
    }
}

impl UsedBinders for XCase {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.clauses.used_binders(used);
    }
}
