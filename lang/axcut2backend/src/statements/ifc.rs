use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    fresh_labels::fresh_label,
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{statements::IfC, TypeDeclaration, TypingContext};

use std::hash::Hash;

impl CodeStatement for IfC {
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
        instructions.push(Backend::comment("ifc".to_string()));
        let fresh_label = format!("lab{}", fresh_label());
        match self.sort {
            axcut::syntax::statements::ifc::IfSort::Equal => Backend::jump_label_if_equal(
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                fresh_label.clone(),
                instructions,
            ),
            axcut::syntax::statements::ifc::IfSort::Less => Backend::jump_label_if_less(
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                fresh_label.clone(),
                instructions,
            ),
        }

        self.elsec
            .code_statement::<Backend, _, _, _>(types, context.clone(), instructions);
        instructions.push(Backend::label(fresh_label));
        self.thenc
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
