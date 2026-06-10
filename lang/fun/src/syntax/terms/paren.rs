//! This module defines parenthesized terms.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::ConstraintBank;
use crate::typing::inference::Inference;
use crate::typing::*;

use std::collections::HashMap;
use std::{collections::HashSet, rc::Rc};

/// This struct defines a term in parentheses.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Paren {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The inner term
    pub inner: Rc<Term>,
}

impl Paren {
    /// This function creates a parenthesized term from a given term.
    pub fn mk<T: Into<Term>>(tm: T) -> Self {
        use crate::syntax::util::dummy_span;

        Paren {
            span: dummy_span(),
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

impl Inference for Paren {
    fn gather_constraints(
            &mut self,
            constraint_bank: &mut ConstraintBank,
            context: &TypingContext,
            ty_var: Ty
        ) -> Result<(), Error> {
        self.inner.gather_constraints(constraint_bank, context, ty_var)
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>
    ) -> Result<(), Error> {

        self.inner.insert_inferred_type(mappings, symbol_table, choices)
    }
}

impl UsedBinders for Paren {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.inner.used_binders(used);
    }
}
