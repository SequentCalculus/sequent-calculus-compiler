use super::CodeStatement;
use crate::code::Code;
use crate::config::{variable_register, RegisterNumber::Snd};
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
        let destination_register = variable_register(Snd, &context, &self.var);
        match self.op {
            BinOp::Prod => {
                instructions.push(Code::MUL(
                    destination_register,
                    variable_register(Snd, &context, &self.fst),
                    variable_register(Snd, &context, &self.snd),
                ));
            }
            BinOp::Sum => {
                instructions.push(Code::ADD(
                    destination_register,
                    variable_register(Snd, &context, &self.fst),
                    variable_register(Snd, &context, &self.snd),
                ));
            }
            BinOp::Sub => {
                instructions.push(Code::SUB(
                    destination_register,
                    variable_register(Snd, &context, &self.fst),
                    variable_register(Snd, &context, &self.snd),
                ));
            }
        }
        self.case.code_statement(types, context, instructions);
    }
}
