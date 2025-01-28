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
    pub term: Rc<Term<Prd>>,
    pub case: Rc<Statement>,
}

impl PrintLnI64 {
    pub fn new<T, U, V>(term: T, next: U) -> PrintLnI64
    where
        T: Into<Term<Prd>>,
        U: Into<Statement>,
    {
        PrintLnI64 {
            term: Rc::new(term.into()),
            case: Rc::new(next.into()),
        }
    }
}

impl Typed for PrintLnI64 {
    fn get_type(&self) -> Ty {
        self.case.get_type()
    }
}

impl Print for PrintLnI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(PRINTLN_I64).append(
            self.term
                .print(cfg, alloc)
                .parens()
                .append(SEMI)
                .append(alloc.line())
                .append(self.case.print(cfg, alloc)),
        )
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
            term: self.term.subst_sim(prod_subst, cons_subst),
            case: self.case.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Uniquify for PrintLnI64 {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> PrintLnI64 {
        PrintLnI64 {
            term: self.term.uniquify(seen_vars, used_vars),
            case: self.case.uniquify(seen_vars, used_vars),
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
                case: self.case.focus(used_vars),
            }
            .into()
        });

        Rc::unwrap_or_clone(self.term).bind(cont, used_vars)
    }
}

/// Focused PrintLnI64
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsPrintLnI64 {
    pub var: Var,
    pub case: Rc<FsStatement>,
}

impl Print for FsPrintLnI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(PRINTLN_I64).append(
            alloc
                .text(&self.var)
                .append(SEMI)
                .append(alloc.space())
                .append(self.case.print(cfg, alloc))
                .parens(),
        )
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
            case: self.case.subst_sim(subst),
        }
    }
}
