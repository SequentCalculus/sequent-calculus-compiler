use super::config::{Immediate, Register, TEMP, ZERO};
use super::Backend;

use axcut::syntax::Name;
use axcut2backend::code::Instructions;

use std::fmt;

#[derive(Debug, Clone)]
pub enum Code {
    ADD(Register, Register, Register),
    ADDI(Register, Register, Immediate),
    SUB(Register, Register, Register),
    MUL(Register, Register, Register),
    DIV(Register, Register, Register),
    REM(Register, Register, Register),
    JAL(Register, String),
    JALR(Register, Register, Immediate),
    LA(Register, String),
    LI(Register, Immediate),
    MV(Register, Register),
    LW(Register, Register, Immediate),
    SW(Register, Register, Immediate),
    BEQ(Register, Register, String),
    BLT(Register, Register, String),
    LAB(String),
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Code::ADD(x, y, z) => write!(f, "ADD {x} {y} {z}"),
            Code::ADDI(x, y, c) => write!(f, "ADD {x} {y} {c}"),
            Code::SUB(x, y, z) => write!(f, "SUB {x} {y} {z}"),
            Code::MUL(x, y, z) => write!(f, "MUL {x} {y} {z}"),
            Code::DIV(x, y, z) => write!(f, "MUL {x} {y} {z}"),
            Code::REM(x, y, z) => write!(f, "REM {x} {y} {z}"),
            Code::JAL(x, l) => write!(f, "JAL {x} {l}"),
            Code::JALR(x, y, c) => write!(f, "JALR {x} {y} {c}"),
            Code::LA(x, l) => write!(f, "LA {x} {l}"),
            Code::LI(x, c) => write!(f, "LI {x} {c}"),
            Code::MV(x, y) => write!(f, "MV {x} {y}"),
            Code::LW(x, y, c) => write!(f, "LW {x} {c} {y}"),
            Code::SW(x, y, c) => write!(f, "SW {x} {c} {y}"),
            Code::BEQ(x, y, l) => write!(f, "BEQ {x} {y} {l}"),
            Code::BLT(x, y, l) => write!(f, "BLT {x} {y} {l}"),
            Code::LAB(l) => write!(f, "\n{l}:"),
        }
    }
}

impl Instructions<Code, Register, Immediate> for Backend {
    fn label(&self, name: Name) -> Code {
        Code::LAB(name)
    }

    fn jump(&self, temporary: Register, instructions: &mut Vec<Code>) {
        instructions.push(Code::JALR(ZERO, temporary, 0));
    }

    fn jump_label(&self, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::JAL(ZERO, name));
    }

    fn jump_label_if_zero(&self, temporary: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::BEQ(temporary, ZERO, name));
    }

    fn load_immediate(
        &self,
        temporary: Register,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::LI(temporary, immediate));
    }

    fn load_label(&self, temporary: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::LA(temporary, name));
    }

    fn add_and_jump(
        &self,
        temporary: Register,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::ADDI(TEMP, temporary, immediate));
        instructions.push(Code::JALR(ZERO, TEMP, 0));
    }

    fn add(
        &self,
        target_temporary: Register,
        source_temporary_1: Register,
        source_temporary_2: Register,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::ADD(
            target_temporary,
            source_temporary_1,
            source_temporary_2,
        ));
    }

    fn sub(
        &self,
        target_temporary: Register,
        source_temporary_1: Register,
        source_temporary_2: Register,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::SUB(
            target_temporary,
            source_temporary_1,
            source_temporary_2,
        ));
    }

    fn mul(
        &self,
        target_temporary: Register,
        source_temporary_1: Register,
        source_temporary_2: Register,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::MUL(
            target_temporary,
            source_temporary_1,
            source_temporary_2,
        ));
    }

    fn div(
        &self,
        target_temporary: Register,
        source_temporary_1: Register,
        source_temporary_2: Register,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::DIV(
            target_temporary,
            source_temporary_1,
            source_temporary_2,
        ));
    }

    fn rem(
        &self,
        target_temporary: Register,
        source_temporary_1: Register,
        source_temporary_2: Register,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::REM(
            target_temporary,
            source_temporary_1,
            source_temporary_2,
        ));
    }

    fn mov(
        &self,
        target_temporary: Register,
        source_temporary: Register,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::MV(target_temporary, source_temporary));
    }
}
