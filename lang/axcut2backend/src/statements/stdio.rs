use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{
    statements::{ReadStdin, WriteStdout},
    Chirality, ContextBinding, Ty, TypeDeclaration, TypingContext,
};

use std::hash::Hash;

impl CodeStatement for ReadStdin {
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
        let comment = format!("{} <- read_stdin {} {};", self.var, self.buffer, self.count);
        instructions.push(Backend::comment(comment));

        let first_free_position = 2 * context.bindings.len();
        context.bindings.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        });
        Backend::read_stdin(
            Backend::variable_temporary(Snd, &context, &self.buffer),
            Backend::variable_temporary(Snd, &context, &self.count),
            Backend::variable_temporary(Snd, &context, &self.var),
            first_free_position,
            instructions,
        );
        self.case
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}

impl CodeStatement for WriteStdout {
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
        let comment = format!(
            "{} <- write_stdout {} {};",
            self.var, self.buffer, self.count
        );
        instructions.push(Backend::comment(comment));

        let first_free_position = 2 * context.bindings.len();
        context.bindings.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        });
        Backend::write_stdout(
            Backend::variable_temporary(Snd, &context, &self.buffer),
            Backend::variable_temporary(Snd, &context, &self.count),
            Backend::variable_temporary(Snd, &context, &self.var),
            first_free_position,
            instructions,
        );
        self.case
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
