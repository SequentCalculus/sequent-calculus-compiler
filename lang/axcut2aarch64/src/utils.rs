use super::code::Code;
use super::memory::load;
use super::statements::CodeStatement;
use axcut::syntax::{Clause, Name, Ty, TypeDeclaration, TypingContext};

#[must_use]
pub fn lookup_type_declaration<'a>(ty: &Ty, types: &'a [TypeDeclaration]) -> &'a TypeDeclaration {
    if let Ty::Decl(type_name) = ty {
        let type_declaration = types
            .iter()
            .find(|declaration| declaration.name == *type_name)
            .unwrap_or_else(|| panic!("Type {type_name} not found"));
        type_declaration
    } else {
        panic!("User-defined type cannot be {ty}");
    }
}

#[must_use]
pub fn xtor_position(tag: &Name, type_declaration: &TypeDeclaration) -> usize {
    type_declaration
        .xtors
        .iter()
        .position(|xtor| xtor.name == *tag)
        .unwrap_or_else(|| {
            panic!("Constructor {tag} not found in type declaration {type_declaration}")
        })
}

pub fn code_table(number_of_entries: usize, base_label: &str, instructions: &mut Vec<Code>) {
    for entry in 0..number_of_entries {
        instructions.push(Code::B(base_label.to_string() + &format!("b{entry}")));
    }
}

fn code_clause(
    mut context: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) {
    load(clause.env.clone(), &context, instructions);
    context.append(&mut clause.env);
    clause.case.code_statement(types, context, instructions);
}

fn code_method(
    mut closure_environment: TypingContext,
    mut clause: Clause,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) {
    load(closure_environment.clone(), &clause.env, instructions);
    clause.env.append(&mut closure_environment);
    clause.case.code_statement(types, clause.env, instructions);
}

pub fn code_clauses(
    context: &TypingContext,
    clauses: Vec<Clause>,
    base_label: &str,
    types: &[TypeDeclaration],
    instructions: &mut Vec<Code>,
) {
    for (n, clause) in clauses.into_iter().enumerate() {
        instructions.push(Code::LAB(base_label.to_string() + &format!("b{n}")));
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
    for (n, clause) in clauses.into_iter().enumerate() {
        instructions.push(Code::LAB(base_label.to_string() + &format!("b{n}")));
        code_method(closure_environment.clone(), clause, types, instructions);
    }
}
