use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    fresh_labels::fresh_label,
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{statements::IfE, TypeDeclaration, TypingContext};

use std::hash::Hash;

impl CodeStatement for IfE {
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
        let fresh_label = format!("lab{}", fresh_label());
        backend.jump_label_if_equal(
            backend.variable_temporary(Snd, &context, &self.fst),
            backend.variable_temporary(Snd, &context, &self.snd),
            fresh_label.clone(),
            instructions,
        );
        self.elsec
            .code_statement(types, context.clone(), backend, instructions);
        instructions.push(backend.label(fresh_label));
        self.thenc
            .code_statement(types, context, backend, instructions);
    }
}
