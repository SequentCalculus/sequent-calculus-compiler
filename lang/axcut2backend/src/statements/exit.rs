use printer::tokens::EXIT;

use super::CodeStatement;
use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber::Snd},
    utils::Utils,
};
use axcut::syntax::{TypeDeclaration, TypingContext, statements::Exit};

impl CodeStatement for Exit {
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
        let comment = format!("{EXIT} {}", self.var);
        instructions.push(Backend::comment(comment));

        Backend::mov(
            Backend::return1(),
            Backend::variable_temporary(Snd, &context, &self.var),
            instructions,
        );
        Backend::jump_label("cleanup".to_string(), instructions);
    }
}
