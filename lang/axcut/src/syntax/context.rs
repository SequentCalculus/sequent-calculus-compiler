//! This module defines typing contexts in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{CNS, COLON, EXT, PRD};
use printer::{DocAllocator, Print};

use super::{Ty, Var};
use crate::traits::{linearize::fresh_var, substitution::Subst};

use std::collections::HashSet;

/// This enum encodes the chirality of a variable in a typing context. In our one-sided setting
/// `Prd` represents producers of data types and consumers of codata types and `Cns` represents
/// consumers of data types and producers of Codata types. The naming thus seems to favor the data
/// side, but this is just due to the lack of better names. `Ext` represents variables of
/// external/built-in types like integers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Chirality {
    /// Producer of data or consumer of codata
    Prd,
    /// Consumer of data or producer of codata
    Cns,
    /// External/built-in
    Ext,
}

impl Print for Chirality {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Chirality::Prd => alloc.space().append(alloc.keyword(PRD)),
            Chirality::Cns => alloc.space().append(alloc.keyword(CNS)),
            Chirality::Ext => alloc.space().append(alloc.keyword(EXT)),
        }
    }
}

/// This struct defines a binding in a typing context. It consists of a variable, its [`Chirality`]
/// and its [`Ty`]pe.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct ContextBinding {
    pub var: Var,
    pub chi: Chirality,
    pub ty: Ty,
}

impl Print for ContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.var
            .print(cfg, alloc)
            .append(COLON)
            .append(self.chi.print(cfg, alloc))
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
    }
}

impl Subst for ContextBinding {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> ContextBinding {
        self.var = self.var.subst_sim(subst);
        self
    }
}

/// This struct defines a typing context. It consists of a list of [`ContextBinding`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypingContext {
    pub bindings: Vec<ContextBinding>,
}

impl TypingContext {
    /// This function returns the list of variables in a typing context.
    pub fn vars(&self) -> Vec<Var> {
        self.bindings
            .iter()
            .map(|binding| &binding.var)
            .cloned()
            .collect()
    }

    /// This function returns the list of variables in a typing context.
    pub fn vars_set(&self) -> HashSet<Var> {
        self.bindings
            .iter()
            .map(|binding| &binding.var)
            .cloned()
            .collect()
    }

    /// This function returns an iterator over the variables in a typing context, consuming the
    /// context.
    pub fn into_iter_vars(self) -> impl Iterator<Item = Var> {
        self.bindings.into_iter().map(|binding| binding.var)
    }

    /// This function returns an iterator over the variables in a typing context, cloning the
    /// variables.
    pub fn iter_vars_cloned(&self) -> impl Iterator<Item = Var> {
        self.bindings.iter().map(|binding| binding.var.clone())
    }

    /// This function picks fresh names for variables that are duplicated in a context.
    /// - `context` is the context in which to pick fresh names.
    /// - `clashes` is the set of variables for which a fresh name must be picked if they occur in the
    ///   context.
    /// - `used_vars` is the set of variable names already used somwhere, i.e., which cannot be used as
    ///   fresh name.
    pub fn freshen(
        &self,
        mut clashes: HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> TypingContext {
        let mut new_bindings = Vec::with_capacity(self.bindings.len());
        for binding in &self.bindings {
            if clashes.contains(&binding.var) {
                // if the variable has occurred already we pick a fresh one
                new_bindings.push(ContextBinding {
                    var: fresh_var(used_vars, &binding.var),
                    ty: binding.ty.clone(),
                    chi: binding.chi.clone(),
                });
            } else {
                // otherwise we keep it, but remember that we have seen it already
                clashes.insert(binding.var.clone());
                new_bindings.push(binding.clone());
            }
        }
        new_bindings.into()
    }

    /// This function keeps all bindings in a context which are contained in a given set. It tries to
    /// retain the original positions of as many bindings as possible in the context by moving bindings
    /// at the end to positions of variables that are not retained.
    /// - `context` is the context from which to keep bindings.
    /// - `set` is the set of variables for which to keep bindings.
    pub fn filter_by_set(&self, set: &HashSet<Var>) -> TypingContext {
        let mut new_context = self.bindings.to_owned();
        for (pos, binding) in self.bindings.iter().enumerate() {
            // if we are beyond the length of the new context, we must have moved all variables from
            // this point on already, so we are done
            if pos >= new_context.len() {
                break;
            } else if !set.contains(&binding.var) {
                // if we do not keep the binding at the current position, we look for one to keep from
                // the end of the new context
                let mut found_element = false;
                while new_context.len() - 1 > pos {
                    if set.contains(&new_context[new_context.len() - 1].var) {
                        found_element = true;
                        // if we have found a binding to keep at the end, we move it to the free
                        // position ...
                        new_context.swap_remove(pos);
                        // ... and stop searching
                        break;
                    }
                    // if we do not keep the binding currently at the end, we remove it
                    new_context.pop();
                }
                if !found_element {
                    // if we do not keep any of the bindings beyond the current position, we simply
                    // remove the one at the current position (and are done now)
                    new_context.pop();
                    break;
                }
            }
        }
        new_context.into()
    }
}

impl Print for TypingContext {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.bindings.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.bindings.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
        }
    }
}

impl From<Vec<ContextBinding>> for TypingContext {
    fn from(bindings: Vec<ContextBinding>) -> Self {
        TypingContext { bindings }
    }
}

impl Subst for TypingContext {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> TypingContext {
        self.bindings = self.bindings.subst_sim(subst);
        self
    }
}
