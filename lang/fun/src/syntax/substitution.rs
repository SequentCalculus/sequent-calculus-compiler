use super::{terms::Term, Covariable};
use std::{collections::HashSet, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubstitutionBinding {
    TermBinding(Term),
    CovarBinding(Covariable),
}

pub type Substitution = Vec<SubstitutionBinding>;

impl fmt::Display for SubstitutionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubstitutionBinding::TermBinding(t) => t.fmt(f),
            SubstitutionBinding::CovarBinding(cv) => write!(f, "'{cv}"),
        }
    }
}

pub fn subst_covars(subst: &Substitution) -> HashSet<Covariable> {
    subst
        .iter()
        .filter_map(|bnd| match bnd {
            SubstitutionBinding::CovarBinding(cv) => Some(cv.clone()),
            _ => None,
        })
        .collect()
}

impl<T: Into<Term>> From<T> for SubstitutionBinding {
    fn from(t: T) -> SubstitutionBinding {
        SubstitutionBinding::TermBinding(t.into())
    }
}

#[cfg(test)]
mod substitution_tests {
    use super::SubstitutionBinding;
    use crate::syntax::terms::Var;

    #[test]
    fn display_term() {
        let result = format!("{}", SubstitutionBinding::TermBinding(Var::mk("x").into()));
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_cv() {
        let result = format!("{}", SubstitutionBinding::CovarBinding("a".to_owned()));
        let expected = "'a";
        assert_eq!(result, expected)
    }
}
