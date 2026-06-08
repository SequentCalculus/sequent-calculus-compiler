//! This module defines integer literals in Fun.

use std::collections::HashMap;

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::typing::inference::{Constraint, ConstraintBank, Inference};
use crate::typing::*;

/// This struct defines integer literals in Fun.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Lit {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The integer value
    pub lit: i64,
}

impl Lit {
    /// This function creates a literal term from a given integer.
    pub fn mk(lit: i64) -> Self {
        use crate::syntax::util::dummy_span;

        Lit {
            span: dummy_span(),
            lit,
        }
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

impl Inference for Lit {
    fn gather_constraints(
            &mut self,
            constraint_bank: &mut ConstraintBank,
            _context: &TypingContext,
            ty_var: Ty
        ) -> Result<(), Error> {
        constraint_bank.constraints.push(Constraint::mk_only_ty(ty_var, Ty::mk_i64()));

        Ok(())
    }

    fn insert_inferred_type(
        &mut self,
        _mappings: &HashMap<Name, Ty>,
        _symbol_table: &mut SymbolTable,
        _choices: &HashMap<Name, usize>
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::*;
    use crate::typing::inference::{Constraint, ConstraintBank, Inference};

    #[test]
    fn inference_lit() {
        let mut term = Lit::mk(15);

        let mut constraint_bank = ConstraintBank{
            symbol_table: Default::default(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(&mut constraint_bank, &TypingContext::default(), Ty::mk_ty_var("x")).unwrap();

        let expected = vec![Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64())];

        let ConstraintBank { constraints: result, .. } = constraint_bank;

        assert_eq!(result, expected);
    }
}
