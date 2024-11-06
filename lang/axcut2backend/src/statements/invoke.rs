use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    utils::Utils,
};
use axcut::syntax::{
    declaration::{lookup_type_declaration, xtor_position},
    Invoke, TypeDeclaration, TypingContext,
};

impl CodeStatement for Invoke {
    fn code_statement<Backend, Code, Temporary, Immediate>(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        backend: &Backend,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Utils<Temporary>,
    {
        let table_temporary = backend.variable_temporary(Snd, &context, &self.var);
        let type_declaration = lookup_type_declaration(&self.ty, types);
        let number_of_clauses = type_declaration.xtors.len();
        if number_of_clauses <= 1 {
            backend.jump(table_temporary, instructions);
        } else {
            let tag_position = xtor_position(&self.tag, type_declaration);
            backend.add_and_jump(
                table_temporary,
                backend.jump_length(tag_position),
                instructions,
            );
        }
    }
}
