use super::CodeStatement;
use crate::code::Code;
use crate::config::{variable_register, RegisterNumber::Snd, TEMP};
use crate::fresh_labels::fresh_label;
use crate::utils::{code_clauses, code_table};
use axcut::syntax::{Switch, TypeDeclaration, TypingContext};

impl CodeStatement for Switch {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let fresh_label = format!("lab{}", fresh_label());
        let number_of_clauses = self.clauses.len();
        instructions.push(Code::ADR(TEMP, fresh_label.clone()));
        let tag_register = variable_register(Snd, &context, &self.var);
        if number_of_clauses <= 1 {
        } else {
            instructions.push(Code::ADD(TEMP, TEMP, tag_register));
        }
        instructions.push(Code::BR(TEMP));
        instructions.push(Code::LAB(fresh_label.clone()));
        if number_of_clauses <= 1 {
        } else {
            code_table(number_of_clauses, &fresh_label, instructions);
        }
        context.pop();
        code_clauses(&context, self.clauses, &fresh_label, types, instructions);
    }
}
