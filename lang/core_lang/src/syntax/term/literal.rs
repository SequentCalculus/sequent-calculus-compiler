use printer::{DocAllocator, Print};

use super::{Cns, FsTerm, Mu, Prd, Term};
use crate::{
    syntax::{fresh_var, statement::FsCut, types::Ty, Covar, FsStatement, Var},
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
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let new_var = fresh_var(used_vars);
        FsCut::new(
            self,
            Mu::tilde_mu(&new_var, k(new_var.clone(), used_vars), Ty::I64),
            Ty::I64,
        )
        .into()
    }
}

#[cfg(test)]
mod lit_tests {
    use super::Bind;
    use super::{Literal, Subst};
    use crate::{
        syntax::{statement::FsCut, term::Mu, types::Ty, FsStatement},
        test_common::example_subst,
    };

    // Substitution tests

    #[test]
    fn subst_lit() {
        let subst = example_subst();
        let result = Literal::new(1).subst_sim(&subst.0, &subst.1);
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
