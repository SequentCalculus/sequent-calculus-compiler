//! This module defines printing an integer in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{PRINT_I64, PRINTLN_I64, SEMI};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::{TypeInference, TypeInferenceState};
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

use std::{collections::HashSet, rc::Rc};

/// This struct defines printing an integer in Fun. It consists of the information whether a
/// newline should be printed, the term for the integer to print, the remaining statement, and
/// after typechecking also of the inferred type.
///
/// Example:
/// `println_i64(x); 1` prints the integer bound to `x` and a newline and then returns `1`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct PrintI64 {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// Whether to print a newline after the value
    pub newline: bool,
    /// The term for the integer to be printed
    pub arg: Rc<Term>,
    /// The next term after the print
    pub next: Rc<Term>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
}

impl OptTyped for PrintI64 {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for PrintI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let print_i64 = if self.newline { PRINTLN_I64 } else { PRINT_I64 };
        alloc
            .keyword(print_i64)
            .append(
                alloc
                    .line_()
                    .append(self.arg.print(cfg, alloc).group())
                    .nest(cfg.indent)
                    .append(alloc.line_())
                    .parens()
                    .group(),
            )
            .append(SEMI)
            .append(alloc.hardline())
            .append(self.next.print(cfg, alloc).group())
    }
}

impl From<PrintI64> for Term {
    fn from(value: PrintI64) -> Self {
        Term::PrintI64(value)
    }
}

impl TypeInference for PrintI64 {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        self.ty = Some(expected_ty.clone());

        self.arg.collect_constraints(state, context, Ty::mk_i64())?;
        self.next.collect_constraints(state, context, expected_ty)?;

        Ok(())
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.arg
            .insert_inferred_types(type_vars, symbol_table, world)?;
        self.next
            .insert_inferred_types(type_vars, symbol_table, world)?;

        Ty::insert_inferred_annotation(&mut self.ty, self.span, type_vars, symbol_table)
    }
}

impl UsedBinders for PrintI64 {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.arg.used_binders(used);
        self.next.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::syntax::util::dummy_span;
    use crate::syntax::{Lit, PrintI64, Term, Ty, TypingContext};
    use crate::typing::inference::{TypeInference, TypeInferenceState, VariationalConstraint};

    #[test]
    fn inference_print() {
        let ctx = TypingContext::default();

        let mut term = PrintI64 {
            span: dummy_span(),
            newline: false,
            arg: Rc::new(Term::Lit(Lit::mk(5))),
            next: Rc::new(Term::Lit(Lit::mk(7))),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            VariationalConstraint::equal(Ty::mk_i64(), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }
}
