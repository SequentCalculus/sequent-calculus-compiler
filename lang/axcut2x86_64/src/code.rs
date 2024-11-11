use super::config::{stack_offset, Immediate, Register, Temporary, STACK, TEMP};
use super::Backend;

use axcut::syntax::Name;
use axcut2backend::code::Instructions;

use std::fmt;

/// x86-64 Assembly instructions
#[derive(Debug, Clone)]
pub enum Code {
    /// https://www.felixcloutier.com/x86/add
    ADD(Register, Register),
    /// https://www.felixcloutier.com/x86/add
    ADDM(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/add
    ADDI(Register, Immediate),
    /// https://www.felixcloutier.com/x86/add
    ADDIM(Register, Immediate, Immediate),
    /// https://www.felixcloutier.com/x86/sub
    SUB(Register, Register),
    /// https://www.felixcloutier.com/x86/sub
    SUBM(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/imul
    IMUL(Register, Register),
    /// https://www.felixcloutier.com/x86/imul
    IMULM(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/jmp
    JMP(Register),
    /// https://www.felixcloutier.com/x86/jmp
    JMPL(String),
    /// https://www.felixcloutier.com/x86/lea
    LEAL(Register, String),
    /// https://www.felixcloutier.com/x86/mov
    MOV(Register, Register),
    /// https://www.felixcloutier.com/x86/mov
    MOVS(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/mov
    MOVL(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/mov
    MOVI(Register, Immediate),
    /// https://www.felixcloutier.com/x86/mov
    MOVIM(Register, Immediate, Immediate),
    /// https://www.felixcloutier.com/x86/cmp
    CMP(Register, Register),
    /// https://www.felixcloutier.com/x86/cmp
    CMPRM(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/cmp
    CMPMR(Register, Immediate, Register),
    /// https://www.felixcloutier.com/x86/cmp
    CMPI(Register, Immediate),
    /// https://www.felixcloutier.com/x86/cmp
    CMPIM(Register, Immediate, Immediate),
    JEL(String),
    LAB(String),
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Code::ADD(x, y) => write!(f, "add {x}, {y}"),
            Code::ADDM(x, y, c) => write!(f, "add {x}, [{y} + {c}]"),
            Code::ADDI(x, c) => write!(f, "add {x}, {c}"),
            Code::ADDIM(x, c, d) => write!(f, "add qword [{x} + {c}], {d}"),
            Code::SUB(x, y) => write!(f, "sub {x}, {y}"),
            Code::SUBM(x, y, c) => write!(f, "sub {x}, [{y} + {c}]"),
            Code::IMUL(x, y) => write!(f, "imul {x}, {y}"),
            Code::IMULM(x, y, c) => write!(f, "imul {x}, [{y} + {c}]"),
            Code::JMP(x) => write!(f, "jmp {x}"),
            Code::JMPL(l) => write!(f, "jmp {l}"),
            Code::LEAL(x, l) => write!(f, "lea {x}, [rel {l}]"),
            Code::MOV(x, y) => write!(f, "mov {x}, {y}"),
            Code::MOVS(x, y, c) => write!(f, "mov [{y} + {c}], {x}"),
            Code::MOVL(x, y, c) => write!(f, "mov {x}, [{y} + {c}]"),
            Code::MOVI(x, c) => write!(f, "mov {x}, {c}"),
            Code::MOVIM(x, c, d) => write!(f, "mov qword [{x} + {c}], {d}"),
            Code::CMP(x, y) => write!(f, "cmp {x}, {y}"),
            Code::CMPRM(x, y, c) => write!(f, "cmp {x}, [{y} + {c}]"),
            Code::CMPMR(x, c, y) => write!(f, "cmp [{x} + {c}], {y}"),
            Code::CMPI(x, c) => write!(f, "cmp {x}, {c}"),
            Code::CMPIM(x, c, d) => write!(f, "cmp qword [{x} + {c}], {d}"),
            Code::JEL(l) => write!(f, "je {l}"),
            Code::LAB(l) => write!(f, "\n{l}:"),
        }
    }
}

pub fn move_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::MOV(register, source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::MOVL(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn add_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::ADD(register, source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::ADDM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn sub_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::SUB(register, source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::SUBM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn mul_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::IMUL(register, source_register));
        }
        Temporary::Spill(source_position) => {
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
        Temporary::Register(target_register) => {
            if target_temporary != source_temporary_1 {
                move_to_register(target_register, source_temporary_1, instructions);
            }
            op_to_register(target_register, source_temporary_2, instructions);
        }
        Temporary::Spill(target_position) => {
            move_to_register(TEMP, source_temporary_1, instructions);
            op_to_register(TEMP, source_temporary_2, instructions);
            instructions.push(Code::MOVS(TEMP, STACK, stack_offset(target_position)));
        }
    }
}

pub fn compare_immediate(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(register) => instructions.push(Code::CMPI(register, immediate)),
        Temporary::Spill(position) => {
            instructions.push(Code::CMPIM(STACK, stack_offset(position), immediate));
        }
    }
}

impl Instructions<Code, Temporary, Immediate> for Backend {
    fn label(&self, name: Name) -> Code {
        Code::LAB(name)
    }

    fn jump(&self, temporary: Temporary, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::JMP(register)),
            Temporary::Spill(position) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
                instructions.push(Code::JMP(TEMP));
            }
        }
    }

    fn jump_label(&self, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::JMPL(name));
    }

    fn jump_label_if_zero(&self, temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, 0, instructions);
        instructions.push(Code::JEL(name));
    }

    fn load_immediate(
        &self,
        temporary: Temporary,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    ) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::MOVI(register, immediate)),
            Temporary::Spill(position) => {
                instructions.push(Code::MOVIM(STACK, stack_offset(position), immediate));
            }
        }
    }

    fn load_label(&self, temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::LEAL(register, name)),
            Temporary::Spill(position) => {
                instructions.push(Code::LEAL(TEMP, name));
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(position)));
            }
        }
    }

    fn add_and_jump(
        &self,
        temporary: Temporary,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    ) {
        match temporary {
            Temporary::Register(register) => {
                instructions.push(Code::ADDI(register, immediate));
                instructions.push(Code::JMP(register));
            }
            Temporary::Spill(position) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
                instructions.push(Code::ADDI(TEMP, immediate));
                instructions.push(Code::JMP(TEMP));
            }
        }
    }

    fn add(
        &self,
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        op(
            add_to_register,
            target_temporary,
            source_temporary_1,
            source_temporary_2,
            instructions,
        );
    }

    fn sub(
        &self,
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        op(
            sub_to_register,
            target_temporary,
            source_temporary_1,
            source_temporary_2,
            instructions,
        );
    }

    fn mul(
        &self,
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        op(
            mul_to_register,
            target_temporary,
            source_temporary_1,
            source_temporary_2,
            instructions,
        );
    }

    fn mov(
        &self,
        target_temporary: Temporary,
        source_temporary: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        match (source_temporary, target_temporary) {
            (Temporary::Register(source_register), Temporary::Register(target_register)) => {
                instructions.push(Code::MOV(target_register, source_register));
            }
            (Temporary::Register(source_register), Temporary::Spill(target_position)) => {
                instructions.push(Code::MOVS(
                    source_register,
                    STACK,
                    stack_offset(target_position),
                ));
            }
            (Temporary::Spill(source_position), Temporary::Register(target_register)) => {
                instructions.push(Code::MOVL(
                    target_register,
                    STACK,
                    stack_offset(source_position),
                ));
            }
            (Temporary::Spill(source_position), Temporary::Spill(target_position)) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(source_position)));
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(target_position)));
            }
        }
    }
}
