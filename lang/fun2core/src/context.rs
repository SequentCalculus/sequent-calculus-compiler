//! This module defines the trivial translation of typing contexts.

use crate::types::compile_ty;

/// This function converts [chirality in Fun](fun::syntax::context::Chirality) to
/// [chirality in Core](core_lang::syntax::context::Chirality).
pub fn compile_chi(chi: &fun::syntax::context::Chirality) -> core_lang::syntax::context::Chirality {
    match chi {
        fun::syntax::context::Chirality::Prd => core_lang::syntax::context::Chirality::Prd,
        fun::syntax::context::Chirality::Cns => core_lang::syntax::context::Chirality::Cns,
    }
}

/// This function converts [typing contexts in Fun](fun::syntax::context::TypingContext) to
/// [typing contexts in Core](core_lang::syntax::context::TypingContext).
pub fn compile_context(
    context: fun::syntax::context::TypingContext,
) -> core_lang::syntax::context::TypingContext {
    core_lang::syntax::context::TypingContext {
        bindings: context
            .bindings
            .into_iter()
            .map(|binding| core_lang::syntax::context::ContextBinding {
                var: core_lang::syntax::names::Var {
                    name: binding.var,
                    id: 0,
                },
                chi: compile_chi(&binding.chi),
                ty: compile_ty(&binding.ty),
            })
            .collect(),
    }
}
