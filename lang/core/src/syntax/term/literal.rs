use super::{Cns, Mu, Prd, Term};
use crate::{
    syntax::{statement::Cut, types::Ty, Covar, Statement, Var},
    traits::{
        focus::{Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        typed::Typed,
    },
};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
}

impl Literal {
    pub fn new(lit: i64) -> Self {
        Literal { lit }
    }
}

impl Typed for Literal {
    fn get_type(&self) -> Ty {
        Ty::Int()
    }
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

impl Focusing for Literal {
    type Target = Literal;
    fn focus(self, _: &mut FocusingState) -> Self::Target {
        self
    }
}

impl Bind for Literal {
    ///bind(⌜n⌝)[k] = ⟨⌜n⌝ | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement {
        let new_var = state.fresh_var();
        Cut::new(
            Term::Literal(self),
            Ty::Int(),
            Mu::tilde_mu(&new_var, Ty::Int(), k(new_var.clone(), state)),
        )
        .into()
    }
}

#[cfg(test)]
mod lit_tests {
    use super::{Bind, Focusing};
    use super::{Cns, FreeV, Literal, Prd, Subst, Term};
    use crate::syntax::{statement::Cut, term::Mu, types::Ty, Statement};
    use crate::syntax::{term::XVar, Covar, Var};

    // Display tests

    #[test]
    fn display_lit() {
        let result = format!("{}", Literal::new(1));
        let expected = "1".to_owned();
        assert_eq!(result, expected)
    }

    // Free variable tests

    #[test]
    fn free_vars_lit() {
        assert!(Literal::new(1).free_vars().is_empty())
    }
    #[test]
    fn free_covars_lit() {
        assert!(Literal::new(1).free_covars().is_empty())
    }

    // Substitution tests

    #[test]
    fn subst_lit() {
        let prod_subst: Vec<(Term<Prd>, Var)> =
            vec![(XVar::var("y", Ty::Int()).into(), "x".to_owned())];
        let cons_subst: Vec<(Term<Cns>, Covar)> =
            vec![(XVar::covar("b", Ty::Int()).into(), "a".to_owned())];
        let result = Literal::new(1).subst_sim(&prod_subst, &cons_subst);
        let expected = Literal::new(1);
        assert_eq!(result, expected)
    }

    // Focusing tests

    #[test]
    fn focus_lit() {
        let result = Literal::new(1).focus(&mut Default::default());
        let expected = Literal::new(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_lit1() {
        let result = Literal::new(1).bind(
            Box::new(|_, _| Statement::Done(Ty::Int())),
            &mut Default::default(),
        );
        let expected = Cut::new(
            Literal::new(1),
            Ty::Int(),
            Mu::tilde_mu("x0", Ty::Int(), Statement::Done(Ty::Int())),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_lit2() {
        let result = Literal::new(2).bind(
            Box::new(|_, _| Statement::Done(Ty::Int())),
            &mut Default::default(),
        );
        let expected = Cut::new(
            Literal::new(2),
            Ty::Int(),
            Mu::tilde_mu("x0", Ty::Int(), Statement::Done(Ty::Int())),
        )
        .into();
        assert_eq!(result, expected)
    }
}
