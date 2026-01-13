//! This module defines printing an integer in Core.

use printer::tokens::{PRINT_I64, PRINTLN_I64, SEMI};
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This struct defines printing an integer in Core. It consists of the information whether a
/// newline should be printed, the term for the integer to print, and the remaining statement. The
/// type parameters `P` and `S` determine whether this is the unfocused variant (if `P` and `S` are
/// instantiated with [`Term<Prd>`] and [`Statement`], which is the default) or the focused variant
/// (if `P` and `C` is instantiated with [`Var`] and [`FsStatement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintI64<P = Rc<Term<Prd>>, S = Statement> {
    /// Whether to print a newline after the value
    pub newline: bool,
    /// The term for the integer to be printed
    pub arg: P,
    /// The next statement after the print
    pub next: Rc<S>,
}

pub type FsPrintI64 = PrintI64<Var, FsStatement>;

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

impl<P, S> Print for PrintI64<P, S>
where
    P: Print,
    S: Print,
{
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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

impl From<PrintI64> for Statement {
    fn from(value: PrintI64) -> Self {
        Statement::PrintI64(value)
    }
}

impl From<FsPrintI64> for FsStatement {
    fn from(value: FsPrintI64) -> Self {
        FsStatement::PrintI64(value)
    }
}

impl Subst for PrintI64 {
    type Target = PrintI64;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Var, Term<Cns>)],
    ) -> Self::Target {
        self.arg = self.arg.subst_sim(prod_subst, cons_subst);
        self.next = self.next.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl SubstVar for FsPrintI64 {
    type Target = FsPrintI64;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> FsPrintI64 {
        self.arg = self.arg.subst_sim(subst);
        self.next = self.next.subst_sim(subst);
        self
    }
}

impl TypedFreeVars for PrintI64 {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.arg.typed_free_vars(vars);
        self.next.typed_free_vars(vars);
    }
}

impl TypedFreeVars for FsPrintI64 {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.insert(ContextBinding {
            var: self.arg.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
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
                        arg: binding.var,
                        next: self.next.focus(used_vars),
                    }
                    .into()
                },
            ),
            used_vars,
        )
    }
}
