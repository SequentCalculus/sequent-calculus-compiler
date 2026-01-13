//! This module defines the trivial translation on names.

/// This function translates a [Core](core_lang) variable to an [AxCut](axcut) variable.
pub fn shrink_var(var: core_lang::syntax::names::Var) -> axcut::syntax::names::Var {
    axcut::syntax::names::Var {
        name: var.name,
        id: var.id,
    }
}
