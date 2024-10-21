use super::CodeStatement;
use crate::code::Code;
use crate::config::{variable_register, RegisterNumber::Snd, RETURN2};
use axcut::syntax::{Return, TypeDeclaration, TypingContext};

impl CodeStatement for Return {
    fn code_statement(
        self,
        _types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::MOVR(
            RETURN2,
            variable_register(Snd, &context, &self.var),
        ));
        instructions.push(Code::B("cleanup".to_string()));
    }
}
