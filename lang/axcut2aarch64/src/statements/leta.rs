use super::CodeStatement;
use crate::code::Code;
use crate::config::{jump_length, variable_register, RegisterNumber::Snd};
use crate::memory::store;
use crate::utils::{lookup_type_declaration, xtor_position};
use axcut::syntax::{ContextBinding, Leta, Polarity, TypeDeclaration, TypingContext};

impl CodeStatement for Leta {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        let arguments = context.split_off(context.len() - self.args.len());
        store(arguments, &context, instructions);
        let tag_position = xtor_position(&self.tag, lookup_type_declaration(&self.ty, types));
        context.push(ContextBinding {
            var: self.var.clone(),
            pol: Polarity::Prd,
            ty: self.ty,
        });
        let tag_register = variable_register(Snd, &context, &self.var);
        instructions.push(Code::MOVI(tag_register, jump_length(tag_position)));
        self.next.code_statement(types, context, instructions);
    }
}
