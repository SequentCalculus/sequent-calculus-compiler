use super::CodeStatement;
use crate::code::Code;
use crate::substitution::{code_exchange, code_weakening_contraction, transpose};
use axcut::syntax::{Substitute, TypeDeclaration, TypingContext};

impl CodeStatement for Substitute {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let target_map = transpose(&self.rearrange, &context);
        let new_context = self
            .rearrange
            .into_iter()
            .map(|binding| binding.0)
            .collect();
        code_weakening_contraction(&target_map, &context, instructions);
        code_exchange(&target_map, &context, &new_context, instructions);
        self.next.code_statement(types, new_context, instructions);
    }
}
