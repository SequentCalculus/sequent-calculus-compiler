pub mod ifz;
pub mod invoke;
pub mod leta;
pub mod literal;
pub mod new;
pub mod op;
pub mod ret;
pub mod substitute;
pub mod switch;

use super::code::Code;
use axcut::syntax::{Statement, TypeDeclaration, TypingContext};

use std::rc::Rc;

pub trait CodeStatement {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    );
}

impl<T: CodeStatement + Clone> CodeStatement for Rc<T> {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        Rc::unwrap_or_clone(self).code_statement(types, context, instructions);
    }
}

impl CodeStatement for Statement {
    fn code_statement(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        instructions: &mut Vec<Code>,
    ) {
        match self {
            Statement::Substitute(substitute) => {
                substitute.code_statement(types, context, instructions);
            }
            Statement::Call(jump) => instructions.push(Code::B(jump.label)),
            Statement::Leta(leta) => leta.code_statement(types, context, instructions),
            Statement::Switch(switch) => switch.code_statement(types, context, instructions),
            Statement::New(new) => new.code_statement(types, context, instructions),
            Statement::Invoke(invoke) => invoke.code_statement(types, context, instructions),
            Statement::Literal(literal) => literal.code_statement(types, context, instructions),
            Statement::Op(op) => op.code_statement(types, context, instructions),
            Statement::IfZ(ifz) => ifz.code_statement(types, context, instructions),
            Statement::Return(ret) => ret.code_statement(types, context, instructions),
            Statement::Done => instructions.push(Code::B("cleanup".to_string())),
        }
    }
}
