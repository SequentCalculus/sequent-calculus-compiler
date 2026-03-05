//! This module defines the code generation for the exit statement.

use printer::{Print, tokens::EXIT};

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
        let comment = format!("{EXIT} {}", self.var.print_to_string(None));
        instructions.push(Backend::comment(comment));

        Backend::mov(
            Backend::return1(),
            Backend::variable_temporary(Snd, &context, self.var.id),
            instructions,
        );
        Backend::jump_label("cleanup".to_string(), instructions);
    }
}
