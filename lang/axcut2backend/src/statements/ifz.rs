use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    fresh_labels::fresh_label,
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{statements::IfZ, TypeDeclaration, TypingContext};

use std::hash::Hash;

impl CodeStatement for IfZ {
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
        let comment = format!("ifz {} \\{{ ... \\}}", self.ifc);
        instructions.push(Backend::comment(comment));

        let fresh_label = format!("lab{}", fresh_label());
        Backend::jump_label_if_zero(
            Backend::variable_temporary(Snd, &context, &self.ifc),
            fresh_label.clone(),
            instructions,
        );
        self.elsec
            .code_statement::<Backend, _, _, _>(types, context.clone(), instructions);
        instructions.push(Backend::label(fresh_label));
        self.thenc
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
