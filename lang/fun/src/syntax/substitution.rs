use super::{terms::Term, Covariable};
use std::fmt;

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

impl<T: Into<Term>> From<T> for SubstitutionBinding {
    fn from(t: T) -> SubstitutionBinding {
        SubstitutionBinding::TermBinding(t.into())
    }
}

// will be removed again later, but is currently needed for the compilation as core has no
// substitutions yet
pub fn split_subst(subst: Substitution) -> (Vec<Term>, Vec<Covariable>) {
    let mut terms = vec![];
    let mut covars = vec![];
    for bind in subst.into_iter() {
        match bind {
            SubstitutionBinding::TermBinding(t) => terms.push(t),
            SubstitutionBinding::CovarBinding(cv) => covars.push(cv),
        }
    }

    (terms, covars)
}

#[cfg(test)]
mod substitution_tests {
    use super::SubstitutionBinding;

    #[test]
    fn display_cv() {
        let result = format!("{}", SubstitutionBinding::CovarBinding("a".to_owned()));
        let expected = "'a";
        assert_eq!(result, expected)
    }
}
