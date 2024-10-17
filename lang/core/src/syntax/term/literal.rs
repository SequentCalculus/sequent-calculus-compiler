use super::{Cns, Mu, Prd, Term};
use crate::{
    syntax::{statement::Cut, Covar, Statement, Var},
    traits::{
        free_vars::FreeV,
        substitution::Subst,
        transform::{Bind, Continuation, NamingTransformation, TransformState},
    },
};
use std::{collections::HashSet, fmt, rc::Rc};

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
    type Target = Literal;
    fn subst_sim(
        &self,
        _prod_subst: &[(Term<Prd>, Var)],
        _cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        self.clone()
    }
}

impl NamingTransformation for Literal {
    type Target = Literal;
    fn transform(self, _: &mut TransformState) -> Self::Target {
        self
    }
}

impl Bind for Literal {
    ///bind(⌜n⌝)[k] = ⟨⌜n⌝ | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_var = state.fresh_var();
        Cut {
            producer: Rc::new(Term::Literal(self)),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: new_var.clone(),
                    statement: Rc::new(k(new_var, state)),
                }
                .into(),
            ),
        }
        .into()
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
        let expected = example_lit();
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod transform_tests {
    use super::{Bind, NamingTransformation};
    use crate::syntax::{
        statement::Cut,
        term::{Cns, Literal, Mu},
        Statement,
    };
    use std::rc::Rc;

    fn example_lit1() -> Literal {
        Literal { lit: 1 }
    }
    fn example_lit2() -> Literal {
        Literal { lit: 2 }
    }

    #[test]
    fn transform_lit1() {
        let result = example_lit1().transform(&mut Default::default());
        let expected = example_lit1();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_lit2() {
        let result = example_lit2().transform(&mut Default::default());
        let expected = example_lit2();
        assert_eq!(result, expected)
    }
    #[test]
    fn bind_lit1() {
        let result =
            example_lit1().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn bind_lit2() {
        let result =
            example_lit2().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 2 }.into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
