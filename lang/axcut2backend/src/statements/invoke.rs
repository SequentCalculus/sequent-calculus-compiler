use printer::Print;

use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    utils::Utils,
};
use axcut::syntax::{statements::Invoke, TypeDeclaration, TypingContext};

impl CodeStatement for Invoke {
    fn code_statement<Backend, Code, Temporary, Immediate>(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Utils<Temporary>,
    {
        let comment = self.print_to_string(None);
        instructions.push(Backend::comment(comment));

        let table_temporary = Backend::variable_temporary(Snd, &context, &self.var);
        let type_declaration = self.ty.lookup_type_declaration(types);
        let number_of_clauses = type_declaration.xtors.len();
        if number_of_clauses <= 1 {
            Backend::jump(table_temporary, instructions);
        } else {
            let tag_position = type_declaration.xtor_position(&self.tag);
            Backend::add_and_jump(
                table_temporary,
                Backend::jump_length(tag_position),
                instructions,
            );
        }
    }
}
