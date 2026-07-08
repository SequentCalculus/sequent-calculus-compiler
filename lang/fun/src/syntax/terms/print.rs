//! This module defines printing an integer in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{PRINT_I64, PRINTLN_I64, SEMI};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::{Constraint, ConstraintBank, Inference};
use crate::typing::*;

use std::collections::HashMap;
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

impl Inference for PrintI64 {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error> {
        // the term type is set to a type variable for easy type lookup after the unification algorithm
        let new_var_type = constraint_bank.var_name_generator.get_new_ty_var();
        self.ty = Some(new_var_type.clone());

        constraint_bank
            .constraints
            .push(Constraint::mk_only_ty(new_var_type, ty_var.clone()));

        self.arg
            .gather_constraints(constraint_bank, context, Ty::mk_i64())?;
        self.next
            .gather_constraints(constraint_bank, context, ty_var)?;

        Ok(())
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
        self.arg
            .insert_inferred_type(mappings, symbol_table, choices)?;
        self.next
            .insert_inferred_type(mappings, symbol_table, choices)?;

        match &mut self.ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span), symbol_table)
            }
            None => panic!(
                "The Type of the term {:?} is not set after type inference",
                self
            ),
        }
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
    use crate::typing::inference::{Constraint, ConstraintBank, Inference};

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

        let mut constraint_bank = ConstraintBank {
            symbol_table: Default::default(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(&mut constraint_bank, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            Constraint::mk_only_ty(Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),
            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let ConstraintBank {
            constraints: result,
            ..
        } = constraint_bank;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")));
    }
}
