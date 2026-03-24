//! This module defines integer literals in Fun.

use std::collections::HashMap;

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::typing::inference::Inference;
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

impl Inference for Lit {
    fn constraint_equations(
            &mut self,
            _symbol_table: &mut SymbolTable,
            _context: &TypingContext,
            _var_name_generator: &mut inference::VarNameGenerator,
            ty_var: Ty
        ) -> Result<Vec<(Ty,Ty)>, Error> {
        Ok(vec![(ty_var, Ty::mk_i64())])
    }

    fn insert_inferred_type(
        &mut self,
        _mappings: &HashMap<Name, Ty>,
        _symbol_table: &mut SymbolTable
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::*;
    use crate::typing::inference::{Inference, VarNameGenerator};
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

    #[test]
    fn inference_lit() {
        let mut term = Lit::mk(15);

        let result = term.constraint_equations(&mut SymbolTable::default(), &TypingContext::default(), &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let expected = vec![(Ty::mk_ty_var("x"), Ty::mk_i64())];

        assert_eq!(result, expected);
    }
}
