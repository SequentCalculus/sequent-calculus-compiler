use printer::Print;

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
        backend: &Backend,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>,
    {
        let fresh_label = format!("{}{}", self.ty.print_to_string(None), fresh_label());
        let number_of_clauses = self.clauses.len();
        backend.load_label(backend.temp(), fresh_label.clone(), instructions);
        let tag_temporary = backend.variable_temporary(Snd, &context, &self.var);
        if number_of_clauses <= 1 {
        } else {
            backend.add(backend.temp(), backend.temp(), tag_temporary, instructions);
        }
        backend.jump(backend.temp(), instructions);
        instructions.push(backend.label(fresh_label.clone()));
        if number_of_clauses <= 1 {
        } else {
            code_table(&self.clauses, &fresh_label, backend, instructions);
        }
        context.bindings.pop();
        code_clauses(
            &context,
            self.clauses,
            &fresh_label,
            types,
            backend,
            instructions,
        );
    }
}
