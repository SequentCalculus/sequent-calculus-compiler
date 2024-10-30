use super::CodeStatement;
use crate::code::Code;
use crate::config::{variable_temporary, TemporaryNumber::Snd};
use crate::utils::{add_to_register, mul_to_register, op, sub_to_register};
use axcut::syntax::{BinOp, Chirality, ContextBinding, Op, Ty, TypeDeclaration, TypingContext};

impl CodeStatement for Op {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        mut context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        context.push(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::Int,
        });
        let destination_temporary = variable_temporary(Snd, &context, &self.var);
        match self.op {
            BinOp::Prod => op(
                mul_to_register,
                destination_temporary,
                variable_temporary(Snd, &context, &self.fst),
                variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
            BinOp::Sum => op(
                add_to_register,
                destination_temporary,
                variable_temporary(Snd, &context, &self.fst),
                variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
            BinOp::Sub => op(
                sub_to_register,
                destination_temporary,
                variable_temporary(Snd, &context, &self.fst),
                variable_temporary(Snd, &context, &self.snd),
                instructions,
            ),
        }
        self.case.code_statement(types, context, instructions);
    }
}
