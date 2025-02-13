use printer::tokens::PRINTLN_I64;

use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{statements::PrintLnI64, TypeDeclaration, TypingContext};

use std::hash::Hash;

impl CodeStatement for PrintLnI64 {
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
        let comment = format!("{PRINTLN_I64} {};", self.var);
        instructions.push(Backend::comment(comment));

        Backend::println_i64(
            Backend::variable_temporary(Snd, &context, &self.var),
            &context.bindings,
            instructions,
        );
        self.next
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
