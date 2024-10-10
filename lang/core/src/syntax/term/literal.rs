use super::{Cns, Prd, Term};
use crate::{
    syntax::{Covar, Var},
    traits::{free_vars::FreeV, substitution::Subst},
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

impl Subst for Literal {
    type Target = Term<Prd>;
    fn subst_sim(
        &self,
        _prod_subst: &[(Term<Prd>, Var)],
        _cons_subst: &[(Term<Cns>, Covar)],
    ) -> Term<Prd> {
        self.clone().into()
    }
}

#[cfg(test)]
mod lit_tests {
    use super::{Cns, FreeV, Literal, Prd, Subst, Term};
    use crate::syntax::{term::XVar, Covar, Var};
    use std::collections::HashSet;

    fn example_lit() -> Literal {
        Literal { lit: 1 }.into()
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
            }
            .into(),
            "x".to_owned(),
        )]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(
            XVar {
                prdcns: Cns,
                var: "b".to_owned(),
            }
            .into(),
            "a".to_owned(),
        )]
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

    #[test]
    fn subst_lit() {
        let result = example_lit().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = example_lit().into();
        assert_eq!(result, expected)
    }
}
