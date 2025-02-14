use printer::{tokens::SWITCH, Print};

use super::CodeStatement;
use crate::fresh_labels::fresh_label;
use crate::utils::{code_clauses, code_table};
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{statements::Switch, TypeDeclaration, TypingContext};

use std::hash::Hash;

impl CodeStatement for Switch {
    fn code_statement<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>,
    {
        let comment = format!("{SWITCH} {} \\{{ ... \\}};", self.var);
        instructions.push(Backend::comment(comment));

        let fresh_label = format!(
            "{}_{}",
            self.ty
                .print_to_string(None)
                .replace('[', "_")
                .replace(", ", "_")
                .replace(']', ""),
            fresh_label()
        );
        let number_of_clauses = self.clauses.len();
        Backend::load_label(Backend::temp(), fresh_label.clone(), instructions);
        let tag_temporary = Backend::variable_temporary(Snd, &context, &self.var);
        if number_of_clauses <= 1 {
        } else {
            Backend::add(
                Backend::temp(),
                Backend::temp(),
                tag_temporary,
                instructions,
            );
        }
        Backend::jump(Backend::temp(), instructions);

        instructions.push(Backend::label(fresh_label.clone()));
        if number_of_clauses <= 1 {
        } else {
            code_table::<Backend, _, _, _>(&self.clauses, &fresh_label, instructions);
        }
        context.bindings.pop();
        code_clauses::<Backend, _, _, _>(&context, self.clauses, &fresh_label, types, instructions);
    }
}
