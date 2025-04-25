use crate::{
    code::Instructions,
    config::{Config, TemporaryNumber},
    memory::Memory,
    parallel_moves::ParallelMoves,
    statements::CodeStatement,
};
use axcut::syntax::{TypeDeclaration, TypingContext, Var, statements::Clause};

use std::hash::Hash;

pub trait Utils<Temporary> {
    fn variable_temporary(
        number: TemporaryNumber,
        context: &TypingContext,
        variable: &Var,
    ) -> Temporary;
    fn fresh_temporary(number: TemporaryNumber, context: &TypingContext) -> Temporary;
}

pub fn code_table<Backend, Code, Temporary, Immediate>(
    clauses: &Vec<Clause>,
    base_label: &str,
    instructions: &mut Vec<Code>,
) where
    Backend: Instructions<Code, Temporary, Immediate>,
{
    for clause in clauses {
        Backend::jump_label_fixed(
            base_label.to_string() + "_" + &clause.xtor.to_string(),
            instructions,
        );
    }
}

fn code_clause<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    mut context: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    Backend::load(clause.context.clone(), &context, instructions);
    context.bindings.append(&mut clause.context.bindings);
    clause
        .body
        .code_statement::<Backend, _, _, _>(types, context, instructions);
}

fn code_method<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    mut closure_environment: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    Backend::load(closure_environment.clone(), &clause.context, instructions);
    clause
        .context
        .bindings
        .append(&mut closure_environment.bindings);
    clause
        .body
        .code_statement::<Backend, _, _, _>(types, clause.context, instructions);
}

pub fn code_clauses<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    context: &TypingContext,
    clauses: Vec<Clause>,
    base_label: &str,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    for clause in clauses {
        instructions.push(Backend::label(
            base_label.to_string() + "_" + &clause.xtor.to_string(),
        ));
        code_clause::<Backend, _, _, _>(context.clone(), clause, types, instructions);
    }
}

pub fn code_methods<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    closure_environment: &TypingContext,
    clauses: Vec<Clause>,
    base_label: &str,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) where
    Backend: Config<Temporary, Immediate>
        + Instructions<Code, Temporary, Immediate>
        + Memory<Code, Temporary>
        + ParallelMoves<Code, Temporary>
        + Utils<Temporary>,
{
    for clause in clauses {
        instructions.push(Backend::label(
            base_label.to_string() + "_" + &clause.xtor.to_string(),
        ));
        code_method::<Backend, _, _, _>(closure_environment.clone(), clause, types, instructions);
    }
}
