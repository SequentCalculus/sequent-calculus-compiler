//! This module defines integer literals in Core.

use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::HashSet;

/// This struct defines integer literals in Core.
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
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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
    use crate::syntax::*;
    use crate::traits::*;

    use core_macros::{fs_cut, fs_exit, fs_mutilde, lit};
    extern crate self as core_lang;
    // Focusing tests

    #[test]
    fn bind_lit1() {
        let result = lit!(1).bind(
            Box::new(|binding, _| FsStatement::Exit(FsExit::exit(&binding.var))),
            &mut Default::default(),
        );
        let expected = fs_cut!(lit!(1), fs_mutilde!("x0", fs_exit!("x0"))).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_lit2() {
        let result = lit!(2).bind(
            Box::new(|binding, _| FsStatement::Exit(FsExit::exit(&binding.var))),
            &mut Default::default(),
        );
        let expected = fs_cut!(lit!(2), fs_mutilde!("x0", fs_exit!("x0"))).into();
        assert_eq!(result, expected)
    }
}
