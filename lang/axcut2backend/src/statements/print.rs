use printer::tokens::{PRINTLN_I64, PRINT_I64};

use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{statements::PrintI64, TypeDeclaration, TypingContext};

use std::hash::Hash;

impl CodeStatement for PrintI64 {
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
        let print_i64 = if self.newline { PRINTLN_I64 } else { PRINT_I64 };
        let comment = format!("{print_i64} {};", self.var);
        instructions.push(Backend::comment(comment));

        Backend::print_i64(
            self.newline,
            Backend::variable_temporary(Snd, &context, &self.var),
            &context.bindings,
            instructions,
        );
        self.next
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
