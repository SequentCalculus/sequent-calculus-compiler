use super::CodeStatement;
use crate::code::Code;
use crate::substitution::{code_exchange, code_weakening_contraction, transpose};
use axcut::syntax::{
    context::lookup_variable_context, ContextBinding, Substitute, TypeDeclaration, TypingContext,
    Var,
};

impl CodeStatement for Substitute {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let rearrange: Vec<(Var, ContextBinding)> = self
            .rearrange
            .clone()
            .into_iter()
            .map(|(new, old)| {
                (
                    new,
                    lookup_variable_context(&old, context.as_slice()).clone(),
                )
            })
            .collect();
        let target_map = transpose(&rearrange, &context);
        let new_context = self
            .rearrange
            .into_iter()
            .map(|binding| {
                let context_binding = lookup_variable_context(&binding.1, &context);
                ContextBinding {
                    var: binding.0,
                    chi: context_binding.chi.clone(),
                    ty: context_binding.ty.clone(),
                }
            })
            .collect();
        code_weakening_contraction(&target_map, &context, instructions);
        code_exchange(&target_map, &context, &new_context, instructions);
        self.next.code_statement(types, new_context, instructions);
    }
}
