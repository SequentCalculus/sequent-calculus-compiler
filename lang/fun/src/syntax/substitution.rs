use printer::{DocAllocator, Print};

use super::{terms::Term, Covariable};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubstitutionBinding {
    TermBinding(Term),
    CovarBinding(Covariable),
}

pub type Substitution = Vec<SubstitutionBinding>;

impl Print for SubstitutionBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            SubstitutionBinding::TermBinding(term) => term.print(cfg, alloc),
            SubstitutionBinding::CovarBinding(cv) => alloc.text("'").append(cv),
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
    use printer::Print;

    use super::SubstitutionBinding;
    use crate::syntax::terms::Var;

    #[test]
    fn display_term() {
        let result = SubstitutionBinding::TermBinding(Var::mk("x").into())
            .print_to_string(Default::default());
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_cv() {
        let result =
            SubstitutionBinding::CovarBinding("a".to_owned()).print_to_string(Default::default());
        let expected = "'a";
        assert_eq!(result, expected)
    }
}
