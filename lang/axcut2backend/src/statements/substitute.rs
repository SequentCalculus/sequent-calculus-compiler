//! This module defines the code generation for explicit substitutions.

use super::CodeStatement;
use crate::substitution::{code_exchange, code_weakening_contraction, transpose};
use crate::{
    code::Instructions, config::Config, memory::Memory, parallel_moves::ParallelMoves, utils::Utils,
};
use axcut::syntax::{ContextBinding, TypeDeclaration, TypingContext, statements::Substitute};

use std::hash::Hash;

impl CodeStatement for Substitute {
    fn code_statement<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>,
    {
        let mut comment = "substitute ".to_string();
        for (x, y) in &self.rearrange {
            comment.push_str(&format!("({x} := {y})"));
        }
        comment.push(';');
        instructions.push(Backend::comment(comment));

        let target_map = transpose(&self.rearrange, &context);
        let new_context = self
            .rearrange
            .into_iter()
            .map(|(new, old)| {
                let context_binding = context.lookup_variable(&old);
                ContextBinding {
                    var: new,
                    chi: context_binding.chi.clone(),
                    ty: context_binding.ty.clone(),
                }
            })
            .collect::<Vec<_>>()
            .into();

        code_weakening_contraction::<Backend, _, _, _>(&target_map, &context, instructions);

        code_exchange::<Backend, _, _, _>(&target_map, &context, &new_context, instructions);

        self.next
            .code_statement::<Backend, _, _, _>(types, new_context, instructions);
    }
}
