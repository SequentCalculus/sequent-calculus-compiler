//! This module defines the code generation for the invocation of a method of a closure.

use printer::Print;

use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    utils::Utils,
};
use axcut::syntax::{TypeDeclaration, TypingContext, statements::Invoke};

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

        let table_temporary = Backend::variable_temporary(Snd, &context, self.var.id);
        let type_declaration = self.ty.lookup_type_declaration(types);
        let number_of_clauses = type_declaration.xtors.len();
        // A type without xtors has no values, so there is nothing to invoke a method on and this
        // statement can never be reached (see the analogous case in `Switch`). Trapping keeps the
        // jump below from targeting whatever the uninitialized table temporary happens to hold.
        if number_of_clauses == 0 {
            instructions.push(Backend::comment(
                "#no clauses, so this invocation is unreachable".to_string(),
            ));
            Backend::unreachable(instructions);
            return;
        }
        if number_of_clauses == 1 {
            instructions.push(Backend::comment(
                "#there is only one clause, so we can jump there directly".to_string(),
            ));
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
