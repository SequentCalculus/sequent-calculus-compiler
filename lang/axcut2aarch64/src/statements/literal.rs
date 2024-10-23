use super::CodeStatement;
use crate::code::Code;
use crate::config::{variable_register, RegisterNumber::Snd};
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
        instructions.push(Code::MOVI(
            variable_register(Snd, &context, &self.var),
            self.lit,
        ));
        self.case.code_statement(types, context, instructions);
    }
}
