//! This module defines integer literals in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::OptTyped;
use crate::typing::inference::{TypeInference, TypeInferenceState, VariationalConstraint};
use crate::typing::inference::{TypeVarMapping, World};
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

impl TypeInference for Lit {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        _context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        state
            .constraints
            .push(VariationalConstraint::equal(expected_ty, Ty::mk_i64()));

        Ok(())
    }

    fn insert_inferred_types(
        &mut self,
        _type_vars: &TypeVarMapping,
        _symbol_table: &mut SymbolTable,
        _world: &World,
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::*;
    use crate::typing::inference::{TypeInference, TypeInferenceState, VariationalConstraint};

    #[test]
    fn inference_lit() {
        let mut term = Lit::mk(15);

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![VariationalConstraint::equal(
            Ty::mk_ty_var("x"),
            Ty::mk_i64(),
        )];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
    }
}
