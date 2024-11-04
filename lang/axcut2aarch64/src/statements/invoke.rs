use super::CodeStatement;
use crate::code::Code;
use crate::config::{jump_length, RegisterNumber::Snd, TEMP};
use crate::utils::variable_register;
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
        let table_register = variable_register(Snd, &context, &self.var);
        let type_declaration = lookup_type_declaration(&self.ty, types);
        let number_of_clauses = type_declaration.xtors.len();
        if number_of_clauses <= 1 {
            instructions.push(Code::BR(table_register));
        } else {
            let tag_position = xtor_position(&self.tag, type_declaration);
            instructions.push(Code::ADDI(TEMP, table_register, jump_length(tag_position)));
            instructions.push(Code::BR(TEMP));
        }
    }
}
