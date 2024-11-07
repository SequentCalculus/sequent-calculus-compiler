use super::CodeStatement;
use crate::code::Code;
use crate::config::TemporaryNumber::Snd;
use crate::utils::{load_immediate, variable_temporary};
use axcut::syntax::{Chirality, ContextBinding, Literal, Ty, TypeDeclaration, TypingContext};

impl CodeStatement for Literal {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        context.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::Int,
        });
        load_immediate(
            variable_temporary(Snd, &context, &self.var),
            self.lit,
            instructions,
        );
        self.case.code_statement(types, context, instructions);
    }
}
