use crate::types::compile_ty;

/// Helper function converting [fun::syntax::context::Chirality] to
/// [core_lang::syntax::context::Chirality]
pub fn compile_chi(chi: &fun::syntax::context::Chirality) -> core_lang::syntax::context::Chirality {
    match chi {
        fun::syntax::context::Chirality::Prd => core_lang::syntax::context::Chirality::Prd,
        fun::syntax::context::Chirality::Cns => core_lang::syntax::context::Chirality::Cns,
    }
}

/// Helper function converting [fun::syntax::context::TypingContext] to
/// [core_lang::syntax::context::TypingContext]
pub fn compile_context(
    ctx: fun::syntax::context::TypingContext,
) -> core_lang::syntax::context::TypingContext {
    core_lang::syntax::context::TypingContext {
        bindings: ctx
            .bindings
            .into_iter()
            .map(|binding| core_lang::syntax::context::ContextBinding {
                var: binding.var,
                chi: compile_chi(&binding.chi),
                ty: compile_ty(&binding.ty),
            })
            .collect(),
    }
}
