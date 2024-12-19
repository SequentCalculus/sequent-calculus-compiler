use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{
    statements::{GetByte, SetByte},
    Chirality, ContextBinding, Ty, TypeDeclaration, TypingContext,
};

use std::hash::Hash;

impl CodeStatement for GetByte {
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
        let comment = format!("{} <- get_byte {} {};", self.var, self.buffer, self.offset);
        instructions.push(Backend::comment(comment));

        context.bindings.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        });
        Backend::load_byte(
            Backend::variable_temporary(Snd, &context, &self.var),
            Backend::variable_temporary(Snd, &context, &self.buffer),
            Backend::variable_temporary(Snd, &context, &self.offset),
            instructions,
        );
        self.case
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}

impl CodeStatement for SetByte {
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
        let comment = format!("set_byte {} {} {};", self.buffer, self.offset, self.var);
        instructions.push(Backend::comment(comment));

        Backend::store_byte(
            Backend::variable_temporary(Snd, &context, &self.var),
            Backend::variable_temporary(Snd, &context, &self.buffer),
            Backend::variable_temporary(Snd, &context, &self.offset),
            instructions,
        );
        self.case
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
