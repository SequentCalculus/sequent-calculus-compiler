use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{
    statements::Op, BinOp, Chirality, ContextBinding, Ty, TypeDeclaration, TypingContext,
};

use std::hash::Hash;

impl CodeStatement for Op {
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
        context.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::Int,
        });
        let target_temporary = backend.variable_temporary(Snd, &context, &self.var);
        match self.op {
            BinOp::Sum => backend.add(
                target_temporary,
                backend.variable_temporary(Snd, &context, &self.fst),
                backend.variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
            BinOp::Sub => backend.sub(
                target_temporary,
                backend.variable_temporary(Snd, &context, &self.fst),
                backend.variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
            BinOp::Prod => backend.mul(
                target_temporary,
                backend.variable_temporary(Snd, &context, &self.fst),
                backend.variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
        }
        self.case
            .code_statement(types, context, backend, instructions);
    }
}
