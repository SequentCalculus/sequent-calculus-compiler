//! Defines [Paren]
use codespan::Span;
use derivative::Derivative;
use printer::Print;

use super::Term;
use crate::{
    syntax::{
        Var,
        context::TypingContext,
        types::{OptTyped, Ty},
    },
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

/// A term in parentheses
/// Example: `(x)`
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
    pub fn mk<T: Into<Term>>(tm: T) -> Self {
        Paren {
            span: Span::default(),
            inner: Rc::new(tm.into()),
        }
    }
}

impl OptTyped for Paren {
    fn get_type(&self) -> Option<Ty> {
        self.inner.get_type()
    }
}

impl Print for Paren {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.inner.print(cfg, alloc).parens()
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
    use super::Check;
    use crate::{
        syntax::{
            context::TypingContext,
            terms::{Lit, Paren},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };

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
