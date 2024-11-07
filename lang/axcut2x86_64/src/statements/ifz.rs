use super::CodeStatement;
use crate::code::Code;
use crate::config::TemporaryNumber::Snd;
use crate::fresh_labels::fresh_label;
use crate::utils::{compare_immediate, variable_temporary};
use axcut::syntax::{IfZ, TypeDeclaration, TypingContext};

impl CodeStatement for IfZ {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let fresh_label = format!("lab{}", fresh_label());
        compare_immediate(
            variable_temporary(Snd, &context, &self.ifc),
            0,
            instructions,
        );
        instructions.push(Code::JEL(fresh_label.clone()));
        self.elsec
            .code_statement(types, context.clone(), instructions);
        instructions.push(Code::LAB(fresh_label));
        self.thenc.code_statement(types, context, instructions);
    }
}
