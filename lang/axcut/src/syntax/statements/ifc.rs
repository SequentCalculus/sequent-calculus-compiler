//! This module defines the conditionals comparing two integers in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{ELSE, EQQ, GT, GTE, IF, LT, LTE, NEQ, ZERO};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use crate::syntax::{Chirality, ContextBinding, Ident, Statement, Ty, TypingContext};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;
use crate::traits::typed_free_vars::TypedFreeVars;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This enum encodes the comparison operation used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSort {
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `<`
    Less,
    /// `<=`
    LessOrEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterOrEqual,
}

impl Print for IfSort {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            IfSort::Equal => alloc.text(EQQ),
            IfSort::NotEqual => alloc.text(NEQ),
            IfSort::Less => alloc.text(LT),
            IfSort::LessOrEqual => alloc.text(LTE),
            IfSort::Greater => alloc.text(GT),
            IfSort::GreaterOrEqual => alloc.text(GTE),
        }
    }
}

/// This struct defines the conditionals comparing either two variables or one variable to zero in
/// AxCut. It consists of the comparison operation, the first variable and an optional second
/// variable, and the then-branch and else-branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfC {
    pub sort: IfSort,
    pub fst: Ident,
    pub snd: Option<Ident>,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Print for IfC {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let snd = match self.snd {
            None => alloc.text(ZERO),
            Some(ref snd) => snd.print(cfg, alloc),
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(self.sort.print(cfg, alloc))
            .append(alloc.space())
            .append(snd)
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc).group())
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
                    .append(self.elsec.print(cfg, alloc).group())
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
    fn free_vars(mut self, vars: &mut HashSet<Ident>) -> Self {
        self.thenc = self.thenc.free_vars(vars);

        let mut vars_elsec = HashSet::new();
        self.elsec = self.elsec.free_vars(&mut vars_elsec);

        vars.extend(vars_elsec);
        vars.insert(self.fst.clone());
        if let Some(ref snd) = self.snd {
            vars.insert(snd.clone());
        }

        self
    }
}

impl TypedFreeVars for IfC {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.insert(ContextBinding {
            var: self.fst.clone(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        });
        if let Some(ref snd) = self.snd {
            vars.insert(ContextBinding {
                var: snd.clone(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            });
        }
        self.thenc.typed_free_vars(vars);
        self.elsec.typed_free_vars(vars);
    }
}

impl Subst for IfC {
    fn subst_sim(mut self, subst: &[(Ident, Ident)]) -> IfC {
        self.fst = self.fst.subst_sim(subst);
        self.snd = self.snd.subst_sim(subst);

        self.thenc = self.thenc.subst_sim(subst);
        self.elsec = self.elsec.subst_sim(subst);

        self
    }
}

impl Linearizing for IfC {
    type Target = IfC;
    fn linearize(mut self, context: TypingContext, used_vars: &mut HashSet<Ident>) -> IfC {
        // we do not insert an explicit substitution, as there are no new bindings and there will
        // be an explicit substitution in each branch
        self.thenc = self.thenc.linearize(context.clone(), used_vars);
        self.elsec = self.elsec.linearize(context, used_vars);

        self
    }
}
