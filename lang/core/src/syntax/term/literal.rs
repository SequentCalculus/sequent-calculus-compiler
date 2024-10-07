use super::{PrdCns, Term};
use crate::{
    syntax::{Covar, Var},
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt};

// Literal
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal<T: PrdCns> {
    pub prdcns: T,
    pub lit: i64,
}

impl<T: PrdCns> std::fmt::Display for Literal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lit)
    }
}

impl<T: PrdCns> FreeV for Literal<T> {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::new()
    }
}

impl<T: PrdCns> From<Literal<T>> for Term<T> {
    fn from(value: Literal<T>) -> Self {
        Term::Literal(value)
    }
}

impl Subst for Literal {
    type Target = Literal;

    fn subst_sim(
        &self,
        _prod_subst: &[(Producer, Var)],
        _cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        self.clone()
    }
}

#[cfg(test)]
mod literal_tests {
    use crate::{
        syntax::{Consumer, Covar, Covariable, Literal, Producer, Var, Variable},
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::collections::HashSet;

    fn example_lit() -> Literal {
        Literal { lit: 2 }
    }

    fn example_prodsubst() -> Vec<(Producer, Var)> {
        vec![(
            Variable {
                var: "y".to_owned(),
            }
            .into(),
            "x".to_owned(),
        )]
    }

    fn example_conssubst() -> Vec<(Consumer, Covar)> {
        vec![(
            Covariable {
                covar: "b".to_owned(),
            }
            .into(),
            "a".to_owned(),
        )]
    }

    #[test]
    fn display_lit() {
        let result = format!("{}", example_lit());
        let expected = "2".to_owned();
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

    #[test]
    fn subst_lit() {
        let result = example_lit().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = example_lit();
        assert_eq!(result, expected)
    }
}
