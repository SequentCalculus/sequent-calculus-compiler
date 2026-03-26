//! This module defines printing an integer in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{PRINT_I64, PRINTLN_I64, SEMI};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::Inference;
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

impl Check for PrintI64 {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        self.arg = self.arg.check(symbol_table, context, &Ty::mk_i64())?;

        self.next = self.next.check(symbol_table, context, expected)?;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl Inference for PrintI64 {
    fn constraint_equations(
            &mut self,
            symbol_table: &mut SymbolTable,
            context: &TypingContext,
            var_name_generator: &mut inference::VarNameGenerator,
            ty_var: Ty
        ) -> Result<Vec<(Ty,Ty)>, Error> {
        let mut constraints: Vec<(Ty, Ty)> = vec![];

        // the term type is set to a type variable for easy type lookup after the unification algorithm
        let new_var_type = var_name_generator.get_new_ty_var();
        self.ty = Some(new_var_type.clone());

        constraints.push((new_var_type, ty_var.clone()));
        
        constraints.append(&mut self.arg.constraint_equations(symbol_table, context, var_name_generator, Ty::mk_i64())?);
        constraints.append(&mut self.next.constraint_equations(symbol_table, context, var_name_generator, ty_var)?);
        
        Ok(constraints)
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable
    ) -> Result<(), Error> {
        self.arg.insert_inferred_type(mappings, symbol_table)?;
        self.next.insert_inferred_type(mappings, symbol_table)?;

        match &mut self.ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span.clone()), symbol_table)
            },
            None => panic!("The Type of the term {:?} is not set after type inference", self)
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

    use crate::syntax::{Lit, PrintI64, Term, Ty, TypingContext};
    use crate::syntax::util::dummy_span;
    use crate::typing::SymbolTable;
    use crate::typing::inference::{Inference, VarNameGenerator};



    #[test]
    fn inference_print() {
        let ctx = TypingContext::default();

        let mut term = PrintI64{span: dummy_span(), newline: false, arg: Rc::new(Term::Lit(Lit::mk(5))), next: Rc::new(Term::Lit(Lit::mk(7))), ty: None};

        let result = term.constraint_equations(&mut SymbolTable::default(), &ctx, &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let expected = vec![(Ty::mk_ty_var("0"), Ty::mk_ty_var("x")), (Ty::mk_i64(), Ty::mk_i64()), (Ty::mk_ty_var("x"), Ty::mk_i64())];

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")));
    }
}