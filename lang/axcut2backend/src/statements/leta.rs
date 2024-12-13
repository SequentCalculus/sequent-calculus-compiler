use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    memory::Memory,
    parallel_moves::ParallelMoves,
    utils::Utils,
};
use axcut::syntax::{statements::Leta, Chirality, ContextBinding, TypeDeclaration, TypingContext};
use printer::Print;

use std::hash::Hash;

impl CodeStatement for Leta {
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
            "leta {}: {} = {}({});",
            self.var,
            self.ty.print_to_string(None),
            self.tag,
            self.args.print_to_string(None)
        );
        instructions.push(Backend::comment(comment));

        let arguments = context
            .bindings
            .split_off(context.bindings.len() - self.args.len());
        Backend::store(arguments.into(), &context, instructions);
        let tag_position = self
            .ty
            .lookup_type_declaration(types)
            .xtor_position(&self.tag);
        context.bindings.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Prd,
            ty: self.ty,
        });
        let tag_temporary = Backend::variable_temporary(Snd, &context, &self.var);
        Backend::load_immediate(
            tag_temporary,
            Backend::jump_length(tag_position),
            instructions,
        );
        self.next
            .code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}
