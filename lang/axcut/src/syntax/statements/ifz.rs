//! This module defines the conditionals comparing a variable to zero in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{ELSE, EQQ, IF, NEQ, ZERO};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use crate::syntax::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

/// This enum encodes the comparison operation used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfZSort {
    Equal,
    NotEqual,
}

/// This struct defines the conditionals comparing variables in AxCut to zero. It consists of the
/// comparison operation, the variable, and the then-branch and else-branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub sort: IfZSort,
    pub ifc: Var,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Print for IfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let comparison = match self.sort {
            IfZSort::Equal => EQQ,
            IfZSort::NotEqual => NEQ,
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.ifc.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(ZERO)
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
            .append(alloc.space())
            .append(alloc.keyword(ELSE))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.elsec.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}

impl FreeVars for IfZ {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.thenc = self.thenc.free_vars(vars);

        let mut vars_elsec = HashSet::new();
        self.elsec = self.elsec.free_vars(&mut vars_elsec);

        vars.extend(vars_elsec);
        vars.insert(self.ifc.clone());

        self
    }
}

impl Subst for IfZ {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> IfZ {
        self.ifc = self.ifc.subst_sim(subst);

        self.thenc = self.thenc.subst_sim(subst);
        self.elsec = self.elsec.subst_sim(subst);

        self
    }
}

impl Linearizing for IfZ {
    type Target = IfZ;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> IfZ {
        // we do not insert an explicit substitution, as there are no new bindings and there will
        // be an explicit substitution in each branch
        self.thenc = self.thenc.linearize(context.clone(), used_vars);
        self.elsec = self.elsec.linearize(context, used_vars);
        self
    }
}
