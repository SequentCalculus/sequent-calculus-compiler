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
use printer::Print;

use std::hash::Hash;

impl CodeStatement for Op {
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
            "{} <- {} {} {};",
            self.var,
            self.fst,
            self.op.print_to_string(None),
            self.snd
        );
        instructions.push(Backend::comment(comment));

        context.bindings.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::Int,
        });
        let target_temporary = Backend::variable_temporary(Snd, &context, &self.var);
        match self.op {
            BinOp::Sum => Backend::add(
                target_temporary,
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
            BinOp::Sub => Backend::sub(
                target_temporary,
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
            BinOp::Prod => Backend::mul(
                target_temporary,
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
            BinOp::Div => Backend::div(
                target_temporary,
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
            BinOp::Rem => Backend::rem(
                target_temporary,
                Backend::variable_temporary(Snd, &context, &self.fst),
                Backend::variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
        }
        self.case
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
