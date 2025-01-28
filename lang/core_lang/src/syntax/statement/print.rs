use printer::{
    theme::ThemeExt,
    tokens::{PRINTLN_I64, SEMI},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        statement::FsStatement,
        term::{Cns, Prd, Term},
        types::Ty,
    },
    traits::*,
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintLnI64 {
    pub arg: Rc<Term<Prd>>,
    pub next: Rc<Statement>,
}

impl PrintLnI64 {
    pub fn new<T, U, V>(arg: T, next: U) -> PrintLnI64
    where
        T: Into<Term<Prd>>,
        U: Into<Statement>,
    {
        PrintLnI64 {
            arg: Rc::new(arg.into()),
            next: Rc::new(next.into()),
        }
    }
}

impl Typed for PrintLnI64 {
    fn get_type(&self) -> Ty {
        self.next.get_type()
    }
}

impl Print for PrintLnI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(PRINTLN_I64)
            .append(self.arg.print(cfg, alloc).parens())
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<PrintLnI64> for Statement {
    fn from(value: PrintLnI64) -> Self {
        Statement::PrintLnI64(value)
    }
}

impl Subst for PrintLnI64 {
    type Target = PrintLnI64;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        PrintLnI64 {
            arg: self.arg.subst_sim(prod_subst, cons_subst),
            next: self.next.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Uniquify for PrintLnI64 {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> PrintLnI64 {
        PrintLnI64 {
            arg: self.arg.uniquify(seen_vars, used_vars),
            next: self.next.uniquify(seen_vars, used_vars),
        }
    }
}

impl Focusing for PrintLnI64 {
    type Target = FsStatement;
    ///N(println_i64(p); s) = bind(p)[λa.println_i64(a); N(s)]
    fn focus(self, used_vars: &mut HashSet<Var>) -> FsStatement {
        let cont = Box::new(|var, used_vars: &mut HashSet<Var>| {
            FsPrintLnI64 {
                var,
                next: self.next.focus(used_vars),
            }
            .into()
        });

        Rc::unwrap_or_clone(self.arg).bind(cont, used_vars)
    }
}

/// Focused PrintLnI64
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsPrintLnI64 {
    pub var: Var,
    pub next: Rc<FsStatement>,
}

impl Print for FsPrintLnI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(PRINTLN_I64)
            .append(self.var.print(cfg, alloc).parens())
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<FsPrintLnI64> for FsStatement {
    fn from(value: FsPrintLnI64) -> Self {
        FsStatement::PrintLnI64(value)
    }
}

impl SubstVar for FsPrintLnI64 {
    type Target = FsPrintLnI64;
    fn subst_sim(self, subst: &[(Var, Var)]) -> FsPrintLnI64 {
        FsPrintLnI64 {
            var: self.var.subst_sim(subst),
            next: self.next.subst_sim(subst),
        }
    }
}
