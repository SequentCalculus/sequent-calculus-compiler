use super::CodeStatement;
use crate::code::Code;
use crate::config::{variable_temporary, TemporaryNumber::Snd, TEMP};
use crate::fresh_labels::fresh_label;
use crate::utils::add_to_register;
use crate::utils::{code_clauses, code_table};
use axcut::syntax::{Switch, TypeDeclaration, TypingContext};

impl CodeStatement for Switch {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let fresh_label = format!("{}{}", self.ty, fresh_label());
        let number_of_clauses = self.clauses.len();
        instructions.push(Code::LEAL(TEMP, fresh_label.clone()));
        let tag_temporary = variable_temporary(Snd, &context, &self.var);
        if number_of_clauses <= 1 {
        } else {
            add_to_register(TEMP, tag_temporary, instructions);
        }
        instructions.push(Code::JMP(TEMP));
        instructions.push(Code::LAB(fresh_label.clone()));
        if number_of_clauses <= 1 {
        } else {
            code_table(&self.clauses, &fresh_label, instructions);
        }
        context.pop();
        code_clauses(&context, self.clauses, &fresh_label, types, instructions);
    }
}
