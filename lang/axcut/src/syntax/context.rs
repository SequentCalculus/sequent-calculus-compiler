//! This module defines typing contexts in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{CNS, COLON, EXT, PRD};
use printer::{DocAllocator, Print};

use super::{Ty, Var};

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

/// This struct defines a typing context. It consists of a list of [`ContextBinding`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypingContext {
    pub bindings: Vec<ContextBinding>,
}

impl TypingContext {
    /// This function returns the list of variables in a typing context.
    pub fn vars(&self) -> Vec<Var> {
        let mut vars = Vec::with_capacity(self.bindings.len());
        for binding in &self.bindings {
            vars.push(binding.var.clone());
        }
        vars
    }

    /// This function returns a reference to the binding for a variable in a typing context.
    /// - `var` is the variable for which to look up the binding.
    ///
    /// # Panics
    ///
    /// A panic is caused if the variable is not in the context.
    pub fn lookup_variable<'a>(&'a self, var: &str) -> &'a ContextBinding {
        self.bindings
            .iter()
            .find(|binding| var == binding.var)
            .unwrap_or_else(|| {
                panic!(
                    "Variable {var} not found in context {}",
                    self.print_to_string(None)
                )
            })
    }
}

impl Print for TypingContext {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.bindings.is_empty() {
            alloc.nil()
        } else {
            self.bindings.print(cfg, alloc).parens()
        }
    }
}

impl From<Vec<ContextBinding>> for TypingContext {
    fn from(bindings: Vec<ContextBinding>) -> Self {
        TypingContext { bindings }
    }
}
