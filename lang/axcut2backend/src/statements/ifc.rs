use printer::tokens::{EQQ, IF, LT, LTE, NEQ};

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
        use axcut::syntax::statements::ifc::IfSort;
        let comment = match self.sort {
            IfSort::Equal => format!("{IF} {} {EQQ} {} \\{{ ... \\}}", self.fst, self.snd),
            IfSort::NotEqual => format!("{IF} {} {NEQ} {} \\{{ ... \\}}", self.fst, self.snd),
            IfSort::Less => format!("{IF} {} {LT} {} \\{{ ... \\}}", self.fst, self.snd),
            IfSort::LessOrEqual => format!("{IF} {} {LTE} {} \\{{ ... \\}}", self.fst, self.snd),
        };
        instructions.push(Backend::comment(comment));

        let fresh_label = format!("lab{}", fresh_label());
        match self.sort {
            IfSort::Equal => Backend::jump_label_if_equal(
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                fresh_label.clone(),
                instructions,
            ),
            IfSort::NotEqual => Backend::jump_label_if_not_equal(
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                fresh_label.clone(),
                instructions,
            ),
            IfSort::Less => Backend::jump_label_if_less(
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                fresh_label.clone(),
                instructions,
            ),
            IfSort::LessOrEqual => Backend::jump_label_if_less_or_equal(
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
