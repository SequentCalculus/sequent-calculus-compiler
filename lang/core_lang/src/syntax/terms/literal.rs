//! This module defines integer literals in Core.

use printer::{DocAllocator, Print};

use super::{FsTerm, Mu, Prd, Term};
use crate::{
    syntax::{
        Chirality, ContextBinding, FsStatement, Var, fresh_var, statements::FsCut, types::Ty,
    },
    traits::*,
};

use std::collections::HashSet;

/// This struct defines integer literals in Fun.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    /// The integer value
    pub lit: i64,
}

impl Literal {
    /// This function creates a literal term from a given integer.
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

impl Bind for Literal {
    // bind(n)\k] = ⟨ n | ~μx.k(x) ⟩
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let new_var = fresh_var(used_vars);
        let new_binding = ContextBinding {
            var: new_var.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        };
        FsCut::new(
            self,
            Mu::tilde_mu(&new_var, k(new_binding, used_vars), Ty::I64),
            Ty::I64,
        )
        .into()
    }
}

#[cfg(test)]
mod lit_tests {
    use super::Bind;
    use super::Literal;
    use crate::syntax::{
        FsStatement,
        statements::{FsCut, FsExit},
        terms::Mu,
        types::Ty,
    };

    // Focusing tests

    #[test]
    fn bind_lit1() {
        let result = Literal::new(1).bind(
            Box::new(|binding, _| FsStatement::Exit(FsExit::exit(&binding.var))),
            &mut Default::default(),
        );
        let expected = FsCut::new(
            Literal::new(1),
            Mu::tilde_mu("x0", FsStatement::Exit(FsExit::exit("x0")), Ty::I64),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_lit2() {
        let result = Literal::new(2).bind(
            Box::new(|binding, _| FsStatement::Exit(FsExit::exit(&binding.var))),
            &mut Default::default(),
        );
        let expected = FsCut::new(
            Literal::new(2),
            Mu::tilde_mu("x0", FsStatement::Exit(FsExit::exit("x0")), Ty::I64),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
