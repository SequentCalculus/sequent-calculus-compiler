//! This module defines the trivial translation of typing contexts.

use crate::types::{compile_ty, compile_ty_poly};
use core_lang::syntax::names::Identifier;
use std::collections::HashMap;

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
                var: Identifier::new(binding.var),
                chi: compile_chi(&binding.chi),
                ty: compile_ty(&binding.ty),
            })
            .collect(),
    }
}

/// This function converts [typing contexts in Fun](fun::syntax::context::TypingContext) to
/// [typing contexts in Core](core_lang::syntax::context::TypingContext), replacing type
/// parameters with the given core identifiers.
/// - `context` is the Fun typing context to translate.
/// - `type_params` maps Fun type parameter names to fresh Core identifiers.
pub fn compile_context_poly(
    context: fun::syntax::context::TypingContext,
    type_params: &HashMap<String, Identifier>,
) -> core_lang::syntax::context::TypingContext {
    core_lang::syntax::context::TypingContext {
        bindings: context
            .bindings
            .into_iter()
            .map(|binding| core_lang::syntax::context::ContextBinding {
                var: Identifier::new(binding.var),
                chi: compile_chi(&binding.chi),
                ty: compile_ty_poly(&binding.ty, type_params),
            })
            .collect(),
    }
}
