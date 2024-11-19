use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{
    declaration::{lookup_type_declaration, xtor_position},
    statements::Leta,
    Chirality, ContextBinding, TypeDeclaration, TypingContext,
};

use std::hash::Hash;

impl CodeStatement for Leta {
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
        let arguments = context.split_off(context.len() - self.args.len());
        backend.store(arguments, &context, instructions);
        let tag_position = xtor_position(&self.tag, lookup_type_declaration(&self.ty, types));
        context.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Prd,
            ty: self.ty,
        });
        let tag_temporary = backend.variable_temporary(Snd, &context, &self.var);
        backend.load_immediate(
            tag_temporary,
            backend.jump_length(tag_position),
            instructions,
        );
        self.next
            .code_statement(types, context, backend, instructions);
    }
}
