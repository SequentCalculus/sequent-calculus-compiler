use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    utils::Utils,
};
use axcut::syntax::{Return, TypeDeclaration, TypingContext};

impl CodeStatement for Return {
    fn code_statement<Backend, Code, Temporary, Immediate>(
        self,
        _types: &[TypeDeclaration],
        context: TypingContext,
        backend: &Backend,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Utils<Temporary>,
    {
        backend.mov(
            backend.return2(),
            backend.variable_temporary(Snd, &context, &self.var),
            instructions,
        );
        backend.jump_label("cleanup".to_string(), instructions);
    }
}
