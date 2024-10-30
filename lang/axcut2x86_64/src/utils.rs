use super::code::Code;
use super::config::{stack_offset, Immediate, Register, Temporary, STACK, TEMP};
use super::memory::load;
use super::statements::CodeStatement;
use axcut::syntax::{Clause, TypeDeclaration, TypingContext};

pub fn move_from_register(temporary: Temporary, register: Register, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(target_register) => instructions.push(Code::MOV(target_register, register)),
        Temporary::S(target_position) => {
            instructions.push(Code::MOVS(register, STACK, stack_offset(target_position)));
        }
    }
}

pub fn move_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(source_register) => instructions.push(Code::MOV(register, source_register)),
        Temporary::S(source_position) => {
            instructions.push(Code::MOVL(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn add_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(source_register) => instructions.push(Code::ADD(register, source_register)),
        Temporary::S(source_position) => {
            instructions.push(Code::ADDM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn sub_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(source_register) => instructions.push(Code::SUB(register, source_register)),
        Temporary::S(source_position) => {
            instructions.push(Code::SUBM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn mul_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(source_register) => instructions.push(Code::IMUL(register, source_register)),
        Temporary::S(source_position) => {
            instructions.push(Code::IMULM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn op(
    op_to_register: fn(register: Register, temporary: Temporary, instructions: &mut Vec<Code>),
    target_temporary: Temporary,
    source_temporary_1: Temporary,
    source_temporary_2: Temporary,
    instructions: &mut Vec<Code>,
) {
    match target_temporary {
        Temporary::R(target_register) => {
            move_to_register(target_register, source_temporary_1, instructions);
            op_to_register(target_register, source_temporary_2, instructions);
        }
        Temporary::S(target_position) => {
            move_to_register(TEMP, source_temporary_1, instructions);
            op_to_register(TEMP, source_temporary_2, instructions);
            instructions.push(Code::MOVS(TEMP, STACK, stack_offset(target_position)));
        }
    }
}

pub fn jump(temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(register) => instructions.push(Code::JMP(register)),
        Temporary::S(position) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
            instructions.push(Code::JMP(TEMP));
        }
    }
}

pub fn add_and_jump(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(register) => {
            instructions.push(Code::ADDI(register, immediate));
            instructions.push(Code::JMP(register));
        }
        Temporary::S(position) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
            instructions.push(Code::ADDI(TEMP, immediate));
            instructions.push(Code::JMP(TEMP));
        }
    }
}

pub fn compare_immediate(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(register) => instructions.push(Code::CMPI(register, immediate)),
        Temporary::S(position) => {
            instructions.push(Code::CMPIM(STACK, stack_offset(position), immediate));
        }
    }
}

pub fn load_immediate(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(register) => instructions.push(Code::MOVI(register, immediate)),
        Temporary::S(position) => {
            instructions.push(Code::MOVIM(STACK, stack_offset(position), immediate));
        }
    }
}

pub fn load_label(temporary: Temporary, label: String, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::R(register) => instructions.push(Code::LEAL(register, label)),
        Temporary::S(position) => {
            instructions.push(Code::LEAL(TEMP, label));
            instructions.push(Code::MOVS(TEMP, STACK, stack_offset(position)));
        }
    }
}

pub fn code_table(clauses: &Vec<Clause>, base_label: &str, instructions: &mut Vec<Code>) {
    for clause in clauses {
        instructions.push(Code::JMPL(
            base_label.to_string() + &clause.xtor.to_string(),
        ));
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
    for clause in clauses {
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
    for clause in clauses {
        instructions.push(Code::LAB(base_label.to_string() + &clause.xtor.to_string()));
        code_method(closure_environment.clone(), clause, types, instructions);
    }
}
