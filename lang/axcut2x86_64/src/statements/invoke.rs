use super::CodeStatement;
use crate::code::Code;
use crate::config::{jump_length, TemporaryNumber::Snd};
use crate::utils::{add_and_jump, jump, variable_temporary};
use axcut::syntax::{
    declaration::{lookup_type_declaration, xtor_position},
    Invoke, TypeDeclaration, TypingContext,
};

impl CodeStatement for Invoke {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let table_temporary = variable_temporary(Snd, &context, &self.var);
        let type_declaration = lookup_type_declaration(&self.ty, types);
        let number_of_clauses = type_declaration.xtors.len();
        if number_of_clauses <= 1 {
            jump(table_temporary, instructions);
        } else {
            let tag_position = xtor_position(&self.tag, type_declaration);
            add_and_jump(table_temporary, jump_length(tag_position), instructions);
        };
    }
}
