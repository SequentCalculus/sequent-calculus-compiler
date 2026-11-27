//! This module defines parenthesized terms.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::TypeInference;
use crate::typing::inference::TypeInferenceState;
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

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

impl TypeInference for Paren {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        self.inner.collect_constraints(state, context, expected_ty)
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.inner
            .insert_inferred_types(type_vars, symbol_table, world)
    }
}

impl UsedBinders for Paren {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.inner.used_binders(used);
    }
}
