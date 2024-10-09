use super::{PrdCns, Term};
use crate::{
    syntax::{stringify_and_join, substitution::Substitution, Covar, Name, Var},
    traits::free_vars::FreeV,
};
use std::{collections::HashSet, fmt};

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor<T: PrdCns> {
    pub prdcns: T,
    pub id: Name,
    pub args: Substitution,
}

impl<T: PrdCns> std::fmt::Display for Xtor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.args);
        write!(f, "{}({})", self.id, args_joined)
    }
}

impl<T: PrdCns> FreeV for Xtor<T> {
    fn free_vars(&self) -> HashSet<Var> {
        self.args.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.args.free_covars()
    }
}

impl<T: PrdCns> From<Xtor<T>> for Term<T> {
    fn from(value: Xtor<T>) -> Self {
        Term::Xtor(value)
    }
}

#[cfg(test)]
mod xtor_tests {
    use super::{FreeV, Xtor};
    use crate::syntax::{
        substitution::SubstitutionBinding,
        term::{Cns, Prd},
        Covariable, Variable,
    };
    use std::collections::HashSet;

    fn example_constructor() -> Xtor<Prd> {
        Xtor {
            prdcns: Prd,
            id: "Cons".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    Variable {
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ProducerBinding(
                    Variable {
                        var: "xs".to_owned(),
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
        .into()
    }

    fn example_destructor() -> Xtor<Cns> {
        Xtor {
            prdcns: Cns,
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
        .into()
    }

    #[test]
    fn display_const() {
        let result = format!("{}", example_constructor());
        let expected = "Cons(x, xs, 'a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_dest() {
        let result = format!("{}", example_destructor());
        let expected = "Hd(x, 'a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_const() {
        let result = example_constructor().free_vars();
        let expected = HashSet::from(["x".to_owned(), "xs".to_owned()]);
        assert_eq!(result, expected)
    }
    #[test]
    fn free_vars_dest() {
        let result = example_destructor().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_const() {
        let result = example_constructor().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }
    #[test]
    fn free_covars_dest() {
        let result = example_destructor().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }
}
