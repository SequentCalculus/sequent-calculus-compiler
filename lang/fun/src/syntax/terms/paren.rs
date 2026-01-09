//! This module defines parenthesized terms.

use codespan::Span;
use derivative::Derivative;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

/// This struct defines a term in parentheses.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Paren {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The inner term
    pub inner: Rc<Term>,
}

impl Paren {
    /// This function creates a parenthesized term from a given term.
    pub fn mk<T: Into<Term>>(tm: T) -> Self {
        Paren {
            span: Span::default(),
            inner: Rc::new(tm.into()),
        }
    }

    pub fn subst_ty(mut self, mappings: &HashMap<Name, Ty>) -> Self {
        self.inner = Rc::new(Rc::unwrap_or_clone(self.inner).subst_ty(mappings));
        self
    }
}

impl OptTyped for Paren {
    fn get_type(&self) -> Option<Ty> {
        self.inner.get_type()
    }
}

impl Print for Paren {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc
            .line_()
            .append(self.inner.print(cfg, alloc).group())
            .nest(cfg.indent)
            .append(alloc.line_())
            .parens()
    }
}

impl From<Paren> for Term {
    fn from(value: Paren) -> Self {
        Term::Paren(value)
    }
}

impl Check for Paren {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        self.inner = self.inner.check(symbol_table, context, expected)?;
        Ok(self)
    }
}

impl UsedBinders for Paren {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.inner.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::*;
    use crate::typing::*;

    #[test]
    fn check_parens() {
        let result = Paren::mk(Lit::mk(1))
            .check(
                &mut SymbolTable::default(),
                &TypingContext::default(),
                &Ty::mk_i64(),
            )
            .unwrap();
        let expected = Paren::mk(Lit::mk(1));
        assert_eq!(result, expected)
    }
}
