use printer::{DocAllocator, Print};

use super::{Cns, FsTerm, Mu, Prd, Term};
use crate::{
    syntax::{statement::FsCut, types::Ty, Covar, FsStatement, Var},
    traits::*,
};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
}

impl Literal {
    #[must_use]
    pub fn new(lit: i64) -> Self {
        Literal { lit }
    }
}

impl Typed for Literal {
    fn get_type(&self) -> Ty {
        Ty::I64
    }
}

impl Print for Literal {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(format!("{}", self.lit))
    }
}

impl From<Literal> for Term<Prd> {
    fn from(value: Literal) -> Self {
        Term::Literal(value)
    }
}

impl From<Literal> for FsTerm<Prd> {
    fn from(value: Literal) -> Self {
        FsTerm::Literal(value)
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

impl Bind for Literal {
    ///bind(⌜n⌝)[k] = ⟨⌜n⌝ | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        let new_var = state.fresh_var();
        FsCut::new(
            self,
            Mu::tilde_mu(&new_var, k(new_var.clone(), state), Ty::I64),
            Ty::I64,
        )
        .into()
    }
}

#[cfg(test)]
mod lit_tests {

    use super::Bind;
    use super::{Cns, FreeV, Literal, Prd, Subst, Term};
    use crate::syntax::term::Mu;
    use crate::syntax::types::Ty;
    use crate::syntax::{statement::FsCut, term::XVar, Covar, FsStatement, Var};

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
            vec![(XVar::var("y", Ty::I64).into(), "x".to_string())];
        let cons_subst: Vec<(Term<Cns>, Covar)> =
            vec![(XVar::covar("b", Ty::I64).into(), "a".to_string())];
        let result = Literal::new(1).subst_sim(&prod_subst, &cons_subst);
        let expected = Literal::new(1);
        assert_eq!(result, expected)
    }

    // Focusing tests

    #[test]
    fn bind_lit1() {
        let result = Literal::new(1).bind(
            Box::new(|_, _| FsStatement::Done()),
            &mut Default::default(),
        );
        let expected = FsCut::new(
            Literal::new(1),
            Mu::tilde_mu("x0", FsStatement::Done(), Ty::I64),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_lit2() {
        let result = Literal::new(2).bind(
            Box::new(|_, _| FsStatement::Done()),
            &mut Default::default(),
        );
        let expected = FsCut::new(
            Literal::new(2),
            Mu::tilde_mu("x0", FsStatement::Done(), Ty::I64),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
