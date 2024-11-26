pub mod ife;
pub mod ifz;
pub mod invoke;
pub mod leta;
pub mod literal;
pub mod new;
pub mod op;
pub mod ret;
pub mod substitute;
pub mod switch;

use crate::{
    code::Instructions, config::Config, memory::Memory, parallel_moves::ParallelMoves, utils::Utils,
};
use axcut::syntax::{Statement, TypeDeclaration, TypingContext};

use std::hash::Hash;
use std::rc::Rc;

pub trait CodeStatement {
    fn code_statement<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        backend: &Backend,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>;
}

impl<T: CodeStatement + Clone> CodeStatement for Rc<T> {
    fn code_statement<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        backend: &Backend,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>,
    {
        Rc::unwrap_or_clone(self).code_statement(types, context, backend, instructions);
    }
}

impl CodeStatement for Statement {
    fn code_statement<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
        backend: &Backend,
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>,
    {
        match self {
            Statement::Substitute(substitute) => {
                substitute.code_statement(types, context, backend, instructions);
            }
            Statement::Call(call) => backend.jump_label(call.label, instructions),
            Statement::Leta(leta) => leta.code_statement(types, context, backend, instructions),
            Statement::Switch(switch) => {
                switch.code_statement(types, context, backend, instructions);
            }
            Statement::New(new) => new.code_statement(types, context, backend, instructions),
            Statement::Invoke(invoke) => {
                invoke.code_statement(types, context, backend, instructions);
            }
            Statement::Literal(lit) => {
                lit.code_statement(types, context, backend, instructions);
            }
            Statement::Op(op) => op.code_statement(types, context, backend, instructions),
            Statement::IfC(ife) => ife.code_statement(types, context, backend, instructions),
            Statement::IfZ(ifz) => ifz.code_statement(types, context, backend, instructions),
            Statement::Return(ret) => ret.code_statement(types, context, backend, instructions),
            Statement::Done => backend.jump_label("cleanup".to_string(), instructions),
        }
    }
}
