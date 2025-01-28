use printer::theme::ThemeExt;
use printer::tokens::{PRINTLN_I64, SEMI};
use printer::{DocAllocator, Print};

use crate::syntax::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintLnI64 {
    pub var: Var,
    pub case: Rc<Statement>,
}

impl Print for PrintLnI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(PRINTLN_I64)
            .append(&self.var)
            .parens()
            .append(SEMI)
            .append(alloc.line())
            .append(self.case.print(cfg, alloc))
    }
}

impl From<PrintLnI64> for Statement {
    fn from(value: PrintLnI64) -> Self {
        Statement::PrintLnI64(value)
    }
}

impl FreeVars for PrintLnI64 {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.insert(self.var.clone());
    }
}

impl Subst for PrintLnI64 {
    type Target = PrintLnI64;

    fn subst_sim(self, subst: &[(Var, Var)]) -> PrintLnI64 {
        PrintLnI64 {
            var: self.var.subst_sim(subst),
            case: self.case.subst_sim(subst),
        }
    }
}

impl Linearizing for PrintLnI64 {
    type Target = PrintLnI64;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> PrintLnI64 {
        PrintLnI64 {
            var: self.var,
            case: self.case.linearize(context.clone(), used_vars),
        }
    }
}
