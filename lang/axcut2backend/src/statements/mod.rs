pub mod ifc;
pub mod ifz;
pub mod invoke;
pub mod leta;
pub mod literal;
pub mod new;
pub mod op;
pub mod print;
pub mod ret;
pub mod substitute;
pub mod switch;

use printer::tokens::{DONE, JUMP};

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
        instructions: &mut Vec<Code>,
    ) where
        Backend: Config<Temporary, Immediate>
            + Instructions<Code, Temporary, Immediate>
            + Memory<Code, Temporary>
            + ParallelMoves<Code, Temporary>
            + Utils<Temporary>,
    {
        Rc::unwrap_or_clone(self).code_statement::<Backend, _, _, _>(types, context, instructions);
    }
}

impl CodeStatement for Statement {
    fn code_statement<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
        self,
        types: &[TypeDeclaration],
        context: TypingContext,
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
                substitute.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::Call(call) => {
                let comment = format!("{JUMP} {}", call.label);
                instructions.push(Backend::comment(comment));

                Backend::jump_label(call.label, instructions);
            }
            Statement::Leta(leta) => {
                leta.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::Switch(switch) => {
                switch.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::New(new) => {
                new.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::Invoke(invoke) => {
                invoke.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::Literal(lit) => {
                lit.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::Op(op) => {
                op.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::PrintLnI64(print) => {
                print.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::IfC(ifc) => {
                ifc.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::IfZ(ifz) => {
                ifz.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::Return(ret) => {
                ret.code_statement::<Backend, _, _, _>(types, context, instructions);
            }
            Statement::Done => {
                let comment = DONE.to_string();
                instructions.push(Backend::comment(comment));

                Backend::jump_label("cleanup".to_string(), instructions);
            }
        }
    }
}
