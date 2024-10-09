use super::{Prd, Term};
use crate::{
    syntax::{Covar, Var},
    traits::free_vars::FreeV,
};
use std::{collections::HashSet, fmt};

// Literal
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lit)
    }
}

impl FreeV for Literal {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::new()
    }
}

impl From<Literal> for Term<Prd> {
    fn from(value: Literal) -> Self {
        Term::Literal(value)
    }
}

#[cfg(test)]
mod lit_tests {
    use super::{FreeV, Literal};
    use std::collections::HashSet;

    fn example_lit() -> Literal {
        Literal { lit: 1 }.into()
    }

    #[test]
    fn display_lit() {
        let result = format!("{}", example_lit());
        let expected = "1".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_lit() {
        let result = example_lit().free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }
    #[test]
    fn free_covars_lit() {
        let result = example_lit().free_covars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }
}
