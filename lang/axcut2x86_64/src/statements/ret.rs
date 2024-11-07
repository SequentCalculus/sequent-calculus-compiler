use super::CodeStatement;
use crate::code::Code;
use crate::config::{TemporaryNumber::Snd, RETURN2};
use crate::utils::{move_to_register, variable_temporary};
use axcut::syntax::{Return, TypeDeclaration, TypingContext};

impl CodeStatement for Return {
    fn code_statement(
        self,
        _types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        move_to_register(
            RETURN2,
            variable_temporary(Snd, &context, &self.var),
            instructions,
        );
        instructions.push(Code::JMPL("cleanup".to_string()));
    }
}
