use super::CodeStatement;
use crate::code::Code;
use crate::config::{variable_register, RegisterNumber::Snd};
use crate::fresh_labels::fresh_label;
use crate::memory::store;
use crate::utils::{code_methods, code_table};
use axcut::syntax::{ContextBinding, New, Polarity, TypeDeclaration, TypingContext};

impl CodeStatement for New {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let closure_environment = context.split_off(context.len() - self.env.len());
        store(closure_environment.clone(), &context, instructions);
        context.push(ContextBinding {
            var: self.var.clone(),
            pol: Polarity::Cns,
            ty: self.ty,
        });
        let fresh_label = format!("lab{}", fresh_label());
        let table_register = variable_register(Snd, &context, &self.var);
        instructions.push(Code::ADR(table_register, fresh_label.clone()));
        self.next.code_statement(types, context, instructions);
        let number_of_clauses = self.clauses.len();
        instructions.push(Code::LAB(fresh_label.clone()));
        if number_of_clauses <= 1 {
        } else {
            code_table(number_of_clauses, &fresh_label, instructions);
        }
        code_methods(
            &closure_environment,
            self.clauses,
            &fresh_label,
            types,
            instructions,
        );
    }
}
