use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    utils::Utils,
};
use axcut::syntax::{statements::Return, TypeDeclaration, TypingContext};

impl CodeStatement for Return {
    fn code_statement<Backend, Code, Temporary, Immediate>(
        self,
        _types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Utils<Temporary>,
    {
        instructions.push(Backend::comment(format!("return {}", self.var)));
        Backend::mov(
            Backend::return2(),
            Backend::variable_temporary(Snd, &context, &self.var),
            instructions,
        );
        Backend::jump_label("cleanup".to_string(), instructions);
    }
}
