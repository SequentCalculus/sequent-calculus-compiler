use super::CodeStatement;
use crate::substitution::{code_exchange, code_weakening_contraction, transpose};
use crate::{
    code::Instructions, config::Config, memory::Memory, parallel_moves::ParallelMoves, utils::Utils,
};
use axcut::syntax::{
    context::lookup_variable_context, statements::Substitute, ContextBinding, TypeDeclaration,
    TypingContext, Var,
};

use std::hash::Hash;

impl CodeStatement for Substitute {
    fn code_statement<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        backend: &Backend,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>,
    {
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
        code_weakening_contraction(&target_map, &context, backend, instructions);
        code_exchange(&target_map, &context, &new_context, backend, instructions);
        self.next
            .code_statement(types, new_context, backend, instructions);
    }
}
