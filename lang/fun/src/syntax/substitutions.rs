use super::{terms::Term, Covariable};
use std::fmt;

#[derive(Debug)]
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

#[cfg(test)]
mod substitution_tests {
    use super::{SubstitutionBinding, Term};

    #[test]
    fn display_term() {
        let result = format!(
            "{}",
            SubstitutionBinding::TermBinding(Term::Var("x".to_owned()))
        );
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
