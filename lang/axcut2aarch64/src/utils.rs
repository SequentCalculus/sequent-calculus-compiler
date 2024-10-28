use super::code::Code;
use super::memory::load;
use super::statements::CodeStatement;
use axcut::syntax::{Clause, TypeDeclaration, TypingContext};

pub fn code_table(clauses: &Vec<Clause>, base_label: &str, instructions: &mut Vec<Code>) {
    for clause in clauses {
        instructions.push(Code::B(base_label.to_string() + &clause.xtor.to_string()));
    }
}

fn code_clause(
    mut context: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) {
    load(clause.context.clone(), &context, instructions);
    context.append(&mut clause.context);
    clause.case.code_statement(types, context, instructions);
}

fn code_method(
    mut closure_environment: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) {
    load(closure_environment.clone(), &clause.context, instructions);
    clause.context.append(&mut closure_environment);
    clause
        .case
        .code_statement(types, clause.context, instructions);
}

pub fn code_clauses(
    context: &TypingContext,
    clauses: Vec<Clause>,
    base_label: &str,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) {
    for clause in clauses.into_iter() {
        instructions.push(Code::LAB(base_label.to_string() + &clause.xtor.to_string()));
        code_clause(context.clone(), clause, types, instructions);
    }
}

pub fn code_methods(
    closure_environment: &TypingContext,
    clauses: Vec<Clause>,
    base_label: &str,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) {
    for clause in clauses.into_iter() {
        instructions.push(Code::LAB(base_label.to_string() + &clause.xtor.to_string()));
        code_method(closure_environment.clone(), clause, types, instructions);
    }
}
