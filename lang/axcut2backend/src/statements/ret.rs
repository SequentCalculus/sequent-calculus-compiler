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
        let comment = format!("return {}", self.var);
        instructions.push(Backend::comment(comment));

        Backend::mov(
            Backend::return1(),
            Backend::variable_temporary(Snd, &context, &self.var),
            instructions,
        );
        Backend::jump_label("cleanup".to_string(), instructions);
    }
}
