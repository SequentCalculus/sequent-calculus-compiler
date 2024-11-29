use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber},
    memory::Memory,
    parallel_moves::ParallelMoves,
    statements::CodeStatement,
};
use axcut::syntax::{Clause, TypeDeclaration, TypingContext, Var};

use std::hash::Hash;

pub trait Utils<Temporary> {
    fn variable_temporary(
        &self,
        number: TemporaryNumber,
        context: &TypingContext,
        variable: &Var,
    ) -> Temporary;
    fn fresh_temporary(&self, number: TemporaryNumber, context: &TypingContext) -> Temporary;
}

pub fn code_table<Backend, Code, Temporary, Immediate>(
    clauses: &Vec<Clause>,
    base_label: &str,
    backend: &Backend,
    instructions: &mut Vec<Code>,
) where
    Backend: Instructions<Code, Temporary, Immediate>,
{
    for clause in clauses {
        backend.jump_label(
            base_label.to_string() + &clause.xtor.to_string(),
            instructions,
        );
    }
}

fn code_clause<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    mut context: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    backend: &Backend,
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    backend.load(clause.context.clone(), &context, instructions);
    context.bindings.append(&mut clause.context.bindings);
    clause
        .case
        .code_statement(types, context, backend, instructions);
}

fn code_method<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    mut closure_environment: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    backend: &Backend,
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    backend.load(closure_environment.clone(), &clause.context, instructions);
    clause
        .context
        .bindings
        .append(&mut closure_environment.bindings);
    clause
        .case
        .code_statement(types, clause.context, backend, instructions);
}

pub fn code_clauses<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    context: &TypingContext,
    clauses: Vec<Clause>,
    base_label: &str,
    types: &[TypeDeclaration],
    backend: &Backend,
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    for clause in clauses {
        instructions.push(backend.label(base_label.to_string() + &clause.xtor.to_string()));
        code_clause(context.clone(), clause, types, backend, instructions);
    }
}

pub fn code_methods<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    closure_environment: &TypingContext,
    clauses: Vec<Clause>,
    base_label: &str,
    types: &[TypeDeclaration],
    backend: &Backend,
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    for clause in clauses {
        instructions.push(backend.label(base_label.to_string() + &clause.xtor.to_string()));
        code_method(
            closure_environment.clone(),
            clause,
            types,
            backend,
            instructions,
        );
    }
}
