use super::CodeStatement;
use crate::code::Code;
use crate::config::{variable_temporary, TemporaryNumber::Snd};
use crate::fresh_labels::fresh_label;
use crate::memory::store;
use crate::utils::{code_methods, code_table, load_label};
use axcut::syntax::{Chirality, ContextBinding, New, TypeDeclaration, TypingContext};

impl CodeStatement for New {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let closure_environment = context.split_off(context.len() - self.context.len());
        store(closure_environment.clone(), &context, instructions);
        let fresh_label = format!("{}{}", self.ty, fresh_label());
        context.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Cns,
            ty: self.ty,
        });
        let table_temporary = variable_temporary(Snd, &context, &self.var);
        load_label(table_temporary, fresh_label.clone(), instructions);
        self.next.code_statement(types, context, instructions);
        let number_of_clauses = self.clauses.len();
        instructions.push(Code::LAB(fresh_label.clone()));
        if number_of_clauses <= 1 {
        } else {
            code_table(&self.clauses, &fresh_label, instructions);
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
