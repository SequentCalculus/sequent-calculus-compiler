use super::CodeStatement;
use crate::{code::Instructions, config::Config, utils::Utils};
use axcut::syntax::{TypeDeclaration, TypingContext, statements::Unreachable};

impl CodeStatement for Unreachable {
    fn code_statement<Backend, Code, Temporary, Immediate>(
        self,
        _types: &[TypeDeclaration],
        _context: TypingContext,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Utils<Temporary>,
    {
        Backend::unreachable(instructions);
    }
}
