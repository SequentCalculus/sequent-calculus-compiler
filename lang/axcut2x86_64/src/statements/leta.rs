use super::CodeStatement;
use crate::code::Code;
use crate::config::{jump_length, variable_temporary, TemporaryNumber::Snd};
use crate::memory::store;
use crate::utils::load_immediate;
use axcut::syntax::{
    declaration::{lookup_type_declaration, xtor_position},
    Chirality, ContextBinding, Leta, TypeDeclaration, TypingContext,
};

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
            chi: Chirality::Prd,
            ty: self.ty,
        });
        let tag_temporary = variable_temporary(Snd, &context, &self.var);
        load_immediate(tag_temporary, jump_length(tag_position), instructions);
        self.next.code_statement(types, context, instructions);
    }
}
