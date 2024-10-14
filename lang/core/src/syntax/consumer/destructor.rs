use super::{Consumer, Covar, Var};
use crate::{
    syntax::{
        stringify_and_join,
        substitution::Substitution,
        term::{Cns, Prd, Term},
        Name,
    },
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt};

// Destructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destructor {
    pub id: Name,
    pub args: Substitution,
}

impl std::fmt::Display for Destructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.args);
        write!(f, "{}({})", self.id, args_joined)
    }
}

impl FreeV for Destructor {
    fn free_vars(&self) -> HashSet<Var> {
        self.args.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.args.free_covars()
    }
}

impl From<Destructor> for Consumer {
    fn from(value: Destructor) -> Self {
        Consumer::Destructor(value)
    }
}

impl Subst for Destructor {
    type Target = Destructor;

    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        Destructor {
            id: self.id.clone(),
            args: self.args.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod destructor_tests {
    use crate::{
        syntax::{
            substitution::SubstitutionBinding,
            term::{Cns, Prd, Term, XVar},
            Covar, Covariable, Destructor, Var, Variable,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::collections::HashSet;

    fn example_dest() -> Destructor {
        Destructor {
            id: "Hd".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    Variable {
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    Covariable {
                        covar: "a".to_owned(),
                    }
                    .into(),
                ),
            ],
        }
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
    fn display_dest() {
        let result = format!("{}", example_dest());
        let expected = "Hd(x, 'a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_dest() {
        let result = example_dest().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_dest() {
        let result = example_dest().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_dest() {
        let result = example_dest().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Destructor {
            id: "Hd".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    Variable {
                        var: "y".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    Covariable {
                        covar: "b".to_owned(),
                    }
                    .into(),
                ),
            ],
        };
        assert_eq!(result, expected)
    }
}
