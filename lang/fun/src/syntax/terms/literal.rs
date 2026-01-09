//! This module defines integer literals in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::*;
use std::collections::HashMap;

use crate::syntax::*;
use crate::typing::*;

/// This struct defines integer literals in Fun.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Lit {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The integer value
    pub lit: i64,
}

impl Lit {
    /// This function creates a literal term from a given integer.
    pub fn mk(lit: i64) -> Self {
        Lit {
            span: Span::default(),
            lit,
        }
    }

    pub fn subst_ty(self, _: &HashMap<Name, Ty>) -> Self {
        self
    }
}

impl OptTyped for Lit {
    fn get_type(&self) -> Option<Ty> {
        Some(Ty::mk_i64())
    }
}

impl Print for Lit {
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc.text(format!("{}", self.lit))
    }
}

impl From<Lit> for Term {
    fn from(value: Lit) -> Self {
        Term::Lit(value)
    }
}

impl Check for Lit {
    fn check(
        self,
        symbol_table: &mut SymbolTable,
        _context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        check_equality(&self.span, symbol_table, expected, &Ty::mk_i64())?;
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::*;
    use crate::typing::*;

    #[test]
    fn check_lit() {
        let result = Lit::mk(1)
            .check(
                &mut SymbolTable::default(),
                &TypingContext::default(),
                &Ty::mk_i64(),
            )
            .unwrap();
        let expected = Lit::mk(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_lit_fail() {
        let result = Lit::mk(1).check(
            &mut SymbolTable::default(),
            &TypingContext::default(),
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        assert!(result.is_err())
    }
}
