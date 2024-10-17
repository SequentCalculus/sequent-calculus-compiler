use crate::{
    syntax::{
        stringify_and_join,
        substitution::Substitution,
        term::{Cns, Prd, Term},
        Covar, Name, Var,
    },
    traits::{
        focus::{bind_many, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
    },
};
use std::{collections::HashSet, fmt};

use super::Statement;

// Fun
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fun {
    pub name: Name,
    pub args: Substitution,
}

impl std::fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.args);
        write!(f, "{}({})", self.name, args_joined)
    }
}

impl From<Fun> for Statement {
    fn from(value: Fun) -> Self {
        Statement::Fun(value)
    }
}

impl FreeV for Fun {
    fn free_vars(&self) -> HashSet<Var> {
        self.args.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.args.free_covars()
    }
}
impl Subst for Fun {
    type Target = Fun;

    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        Fun {
            name: self.name.clone(),
            args: self.args.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Focusing for Fun {
    type Target = Statement;
    ///N(f(p_i; c_j)) = bind(p_i)[λas.bind(c_j)[λbs.f(as; bs)]]
    fn focus(self, state: &mut FocusingState) -> Statement {
        bind_many(
            self.args.into(),
            Box::new(|args, _: &mut FocusingState| {
                Fun {
                    name: self.name,
                    args: args.into_iter().collect(),
                }
                .into()
            }),
            state,
        )
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::{
        statement::Fun,
        substitution::SubstitutionBinding,
        term::{Cns, Prd, XVar},
    };

    fn example_fun1() -> Fun {
        Fun {
            name: "main".to_owned(),
            args: vec![],
        }
    }
    fn example_fun2() -> Fun {
        Fun {
            name: "fun".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                    }
                    .into(),
                ),
            ],
        }
    }

    #[test]
    fn transform_fun1() {
        let result = example_fun1().focus(&mut Default::default());
        let expected = Fun {
            name: "main".to_owned(),
            args: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_fun2() {
        let result = example_fun2().focus(&mut Default::default());
        let expected = Fun {
            name: "fun".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                    }
                    .into(),
                ),
            ],
        }
        .into();
        assert_eq!(result, expected)
    }
}
