use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{
    statements::Literal, Chirality, ContextBinding, Ty, TypeDeclaration, TypingContext,
};

use std::hash::Hash;

impl CodeStatement for Literal {
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
        let comment = format!("lit {} <- {};", self.var, self.lit);
        instructions.push(Backend::comment(comment));

        context.bindings.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        });
        Backend::load_immediate(
            Backend::variable_temporary(Snd, &context, &self.var),
            Backend::i64_to_immediate(self.lit),
            instructions,
        );

        self.case
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
