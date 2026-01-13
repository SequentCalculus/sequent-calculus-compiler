//! This module defines the trivial translation on names.

/// This function converts [variables in Fun](fun::syntax::names::Var) to
/// [variables in Core](core_lang::syntax::names::Var).
pub fn compile_var(var: fun::syntax::names::Var) -> core_lang::syntax::names::Var {
    core_lang::syntax::names::Var {
        name: var.name,
        id: var.id,
    }
}
