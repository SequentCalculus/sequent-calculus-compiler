//! This module defines the conditionals comparing two variables in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{ELSE, EQQ, IF, LT, LTE, NEQ};
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
pub enum IfSort {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
}

/// This struct defines the conditionals comparing two variables in AxCut. It consists of the
/// comparison operation, the two variables, and the then-branch and else-branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfC {
    pub sort: IfSort,
    pub fst: Var,
    pub snd: Var,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Print for IfC {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let comparison = match self.sort {
            IfSort::Equal => EQQ,
            IfSort::NotEqual => NEQ,
            IfSort::Less => LT,
            IfSort::LessOrEqual => LTE,
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
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

impl From<IfC> for Statement {
    fn from(value: IfC) -> Self {
        Statement::IfC(value)
    }
}

impl FreeVars for IfC {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.thenc = self.thenc.free_vars(vars);

        let mut vars_elsec = HashSet::new();
        self.elsec = self.elsec.free_vars(&mut vars_elsec);

        vars.extend(vars_elsec);
        vars.insert(self.fst.clone());
        vars.insert(self.snd.clone());

        self
    }
}

impl Subst for IfC {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> IfC {
        self.fst = self.fst.subst_sim(subst);
        self.snd = self.snd.subst_sim(subst);

        self.thenc = self.thenc.subst_sim(subst);
        self.elsec = self.elsec.subst_sim(subst);

        self
    }
}

impl Linearizing for IfC {
    type Target = IfC;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> IfC {
        // we do not insert an explicit substitution, as there are no new bindings and there will
        // be an explicit substitution in each branch
        self.thenc = self.thenc.linearize(context.clone(), used_vars);
        self.elsec = self.elsec.linearize(context, used_vars);

        self
    }
}
