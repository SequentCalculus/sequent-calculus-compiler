//! This module defines the code generation for the conditionals comparing two integers.

use printer::tokens::{EQQ, GT, GTE, IF, LT, LTE, NEQ, ZERO};

use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    fresh_labels::fresh_label,
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{TypeDeclaration, TypingContext, statements::IfC};

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
        let snd = match self.snd {
            None => ZERO.to_string(),
            Some(ref snd) => snd.to_string(),
        };
        let comment = match self.sort {
            IfSort::Equal => format!("{IF} {} {EQQ} {snd} \\{{ ... \\}}", self.fst),
            IfSort::NotEqual => format!("{IF} {} {NEQ} {snd} \\{{ ... \\}}", self.fst),
            IfSort::Less => format!("{IF} {} {LT} {snd} \\{{ ... \\}}", self.fst),
            IfSort::LessOrEqual => format!("{IF} {} {LTE} {snd} \\{{ ... \\}}", self.fst),
            IfSort::Greater => format!("{IF} {} {GT} {snd} \\{{ ... \\}}", self.fst),
            IfSort::GreaterOrEqual => format!("{IF} {} {GTE} {snd} \\{{ ... \\}}", self.fst),
        };
        instructions.push(Backend::comment(comment));

        let fresh_label = format!("lab{}", fresh_label());
        match self.snd {
            None => match self.sort {
                IfSort::Equal => Backend::jump_label_if_zero(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::NotEqual => Backend::jump_label_if_not_zero(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::Less => Backend::jump_label_if_less_zero(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::LessOrEqual => Backend::jump_label_if_less_or_equal_zero(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::Greater => Backend::jump_label_if_greater_zero(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::GreaterOrEqual => Backend::jump_label_if_greater_or_equal_zero(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    fresh_label.clone(),
                    instructions,
                ),
            },
            Some(snd) => match self.sort {
                IfSort::Equal => Backend::jump_label_if_equal(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    Backend::variable_temporary(Snd, &context, &snd),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::NotEqual => Backend::jump_label_if_not_equal(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    Backend::variable_temporary(Snd, &context, &snd),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::Less => Backend::jump_label_if_less(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    Backend::variable_temporary(Snd, &context, &snd),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::LessOrEqual => Backend::jump_label_if_less_or_equal(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    Backend::variable_temporary(Snd, &context, &snd),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::Greater => Backend::jump_label_if_greater(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    Backend::variable_temporary(Snd, &context, &snd),
                    fresh_label.clone(),
                    instructions,
                ),
                IfSort::GreaterOrEqual => Backend::jump_label_if_greater_or_equal(
                    Backend::variable_temporary(Snd, &context, &self.fst),
                    Backend::variable_temporary(Snd, &context, &snd),
                    fresh_label.clone(),
                    instructions,
                ),
            },
        }

        instructions.push(Backend::comment("else branch".to_string()));
        self.elsec
            .code_statement::<Backend, _, _, _>(types, context.clone(), instructions);

        instructions.push(Backend::label(fresh_label));
        instructions.push(Backend::comment("then branch".to_string()));
        self.thenc
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
