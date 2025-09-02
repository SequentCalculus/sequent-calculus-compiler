//! This module defines printing an integer in Core.

use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{PRINT_I64, PRINTLN_I64, SEMI},
};

use super::{ContextBinding, Covar, Statement, Var};
use crate::{
    syntax::{
        FsStatement,
        context::Chirality,
        terms::{Cns, Prd, Term},
        types::Ty,
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This struct defines printing an integer in Core. It consists of the information whether a
/// newline should be printed, the term for the integer to print, and the remaining statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintI64 {
    /// Whether to print a newline after the value
    pub newline: bool,
    /// The term for the integer to be printed
    pub arg: Rc<Term<Prd>>,
    /// The next statement after the print
    pub next: Rc<Statement>,
}

impl PrintI64 {
    /// This function creates a new print statement from an argument and a remaining statement.
    pub fn new<T, U, V>(arg: T, next: U, newline: bool) -> PrintI64
    where
        T: Into<Term<Prd>>,
        U: Into<Statement>,
    {
        PrintI64 {
            newline,
            arg: Rc::new(arg.into()),
            next: Rc::new(next.into()),
        }
    }
}

impl Typed for PrintI64 {
    fn get_type(&self) -> Ty {
        self.next.get_type()
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
            .append(self.arg.print(cfg, alloc).parens())
            .append(SEMI)
            .append(alloc.hardline())
            .append(self.next.print(cfg, alloc).group())
    }
}

impl From<PrintI64> for Statement {
    fn from(value: PrintI64) -> Self {
        Statement::PrintI64(value)
    }
}

impl Subst for PrintI64 {
    type Target = PrintI64;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        self.arg = self.arg.subst_sim(prod_subst, cons_subst);
        self.next = self.next.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl TypedFreeVars for PrintI64 {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.arg.typed_free_vars(vars);
        self.next.typed_free_vars(vars);
    }
}

impl Uniquify for PrintI64 {
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> PrintI64 {
        self.arg = self.arg.uniquify(seen_vars, used_vars);
        self.next = self.next.uniquify(seen_vars, used_vars);
        self
    }
}

impl Focusing for PrintI64 {
    type Target = FsStatement;
    // focus(println_i64(p); s) = bind(p)[λa.println_i64(a); focus(s)]
    fn focus(self, used_vars: &mut HashSet<Var>) -> FsStatement {
        Rc::unwrap_or_clone(self.arg).bind(
            Box::new(
                move |binding: ContextBinding, used_vars: &mut HashSet<Var>| {
                    FsPrintI64 {
                        newline: self.newline,
                        var: binding.var,
                        next: self.next.focus(used_vars),
                    }
                    .into()
                },
            ),
            used_vars,
        )
    }
}

/// This struct defines the focused version of the [`PrintI64`] statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsPrintI64 {
    /// Whether to print a newline after the value
    pub newline: bool,
    /// The integer to print (always a variable here)
    pub var: Var,
    /// The next statement after the print
    pub next: Rc<FsStatement>,
}

impl Print for FsPrintI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let print_i64 = if self.newline { PRINTLN_I64 } else { PRINT_I64 };
        alloc
            .keyword(print_i64)
            .append(self.var.print(cfg, alloc).parens())
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<FsPrintI64> for FsStatement {
    fn from(value: FsPrintI64) -> Self {
        FsStatement::PrintI64(value)
    }
}

impl SubstVar for FsPrintI64 {
    type Target = FsPrintI64;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> FsPrintI64 {
        self.var = self.var.subst_sim(subst);
        self.next = self.next.subst_sim(subst);
        self
    }
}

impl TypedFreeVars for FsPrintI64 {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.insert(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
        self.next.typed_free_vars(vars);
    }
}
