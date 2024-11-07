use printer::{tokens::TICK, DocAllocator, Print};

use super::{terms::Term, types::Ty, Covariable};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubstitutionBinding {
    TermBinding { term: Term, ty: Option<Ty> },
    CovarBinding { covar: Covariable, ty: Option<Ty> },
}

pub type Substitution = Vec<SubstitutionBinding>;

impl Print for SubstitutionBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            SubstitutionBinding::TermBinding { term, ty: _ } => term.print(cfg, alloc),
            SubstitutionBinding::CovarBinding { covar, ty: _ } => alloc.text(TICK).append(covar),
        }
    }
}

pub fn subst_covars(subst: &Substitution) -> HashMap<Covariable, Option<Ty>> {
    let mut covar_map = HashMap::new();
    for subst in subst.iter() {
        if let SubstitutionBinding::CovarBinding { covar, ty } = subst {
            covar_map.insert(covar.clone(), ty.clone());
        }
    }
    covar_map
}

impl<T: Into<Term>> From<T> for SubstitutionBinding {
    fn from(t: T) -> SubstitutionBinding {
        SubstitutionBinding::TermBinding {
            term: t.into(),
            ty: None,
        }
    }
}

#[cfg(test)]
mod substitution_tests {
    use printer::Print;

    use super::SubstitutionBinding;
    use crate::syntax::{terms::Var, types::Ty};

    #[test]
    fn display_term() {
        let result = SubstitutionBinding::TermBinding {
            ty: Some(Ty::mk_int()),
            term: Var::mk("x").into(),
        }
        .print_to_string(Default::default());
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_cv() {
        let result = SubstitutionBinding::CovarBinding {
            ty: Some(Ty::mk_int()),
            covar: "a".to_owned(),
        }
        .print_to_string(Default::default());
        let expected = "'a";
        assert_eq!(result, expected)
    }
}
