use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{
    statements::{MMapAnonymousPage, MUnmapPage},
    Chirality, ContextBinding, Ty, TypeDeclaration, TypingContext,
};

use std::hash::Hash;

impl CodeStatement for MMapAnonymousPage {
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
        let comment = format!("{} <- mmap_anonymous_page;", self.var);
        instructions.push(Backend::comment(comment));

        let first_free_position = 2 * context.bindings.len();
        context.bindings.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::Page,
        });
        Backend::mmap_anonymous_page(
            Backend::variable_temporary(Snd, &context, &self.var),
            first_free_position,
            instructions,
        );
        self.case
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}

impl CodeStatement for MUnmapPage {
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
        let comment = format!("munmap_page {};", self.var);
        instructions.push(Backend::comment(comment));

        let first_free_position = 2 * context.bindings.len();
        Backend::munmap_page(
            Backend::variable_temporary(Snd, &context, &self.var),
            first_free_position,
            instructions,
        );
        self.case
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
