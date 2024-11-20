use super::config::{Immediate, Register, TEMP};
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
    SDIV(Register, Register, Register),
    MSUB(Register, Register, Register, Register),
    B(String),
    BR(Register),
    ADR(Register, String),
    MOVR(Register, Register),
    MOVI(Register, Immediate),
    LDR(Register, Register, Immediate),
    STR(Register, Register, Immediate),
    CMPR(Register, Register),
    CMPI(Register, Immediate),
    BEQ(String),
    BLT(String),
    LAB(String),
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Code::ADD(x, y, z) => write!(f, "ADD {x}, {y}, {z}"),
            Code::ADDI(x, y, c) => write!(f, "ADD {x}, {y}, {c}"),
            Code::SUB(x, y, z) => write!(f, "SUB {x}, {y}, {z}"),
            Code::MUL(x, y, z) => write!(f, "MUL {x}, {y}, {z}"),
            Code::SDIV(x, y, z) => write!(f, "SDIV {x}, {y}, {z}"),
            Code::MSUB(x, y, z, v) => write!(f, "MSUB {x}, {y}, {z}, {v}"),
            Code::B(l) => write!(f, "B {l}"),
            Code::BR(x) => write!(f, "BR {x}"),
            Code::ADR(x, l) => write!(f, "ADR {x}, {l}"),
            Code::MOVR(x, y) => write!(f, "MOV {x}, {y}"),
            Code::MOVI(x, c) => write!(f, "MOV {x}, {c}"),
            Code::LDR(x, y, c) => write!(f, "LDR {x}, [ {y}, {c} ]"),
            Code::STR(x, y, c) => write!(f, "STR {x}, [ {y}, {c} ]"),
            Code::CMPR(x, y) => write!(f, "CMP {x}, {y}"),
            Code::CMPI(x, c) => write!(f, "CMP {x}, {c}"),
            Code::BEQ(l) => write!(f, "BEQ {l}"),
            Code::BLT(l) => write!(f, "BLT {l}"),
            Code::LAB(l) => write!(f, "\n{l}:"),
        }
    }
}

impl Instructions<Code, Register, Immediate> for Backend {
    fn label(&self, name: Name) -> Code {
        Code::LAB(name)
    }

    fn jump(&self, temporary: Register, instructions: &mut Vec<Code>) {
        instructions.push(Code::BR(temporary));
    }

    fn jump_label(&self, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::B(name));
    }

    fn jump_label_if_equal(
        &self,
        fst: Register,
        snd: Register,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::CMPR(fst, snd));
        instructions.push(Code::BEQ(name));
    }

    fn jump_label_if_less(
        &self,
        fst: Register,
        snd: Register,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::CMPR(fst, snd));
        instructions.push(Code::BLT(name));
    }

    fn jump_label_if_zero(&self, temporary: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::CMPI(temporary, 0));
        instructions.push(Code::BEQ(name));
    }

    fn load_immediate(
        &self,
        temporary: Register,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::MOVI(temporary, immediate));
    }

    fn load_label(&self, temporary: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::ADR(temporary, name));
    }

    fn add_and_jump(
        &self,
        temporary: Register,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::ADDI(TEMP, temporary, immediate));
        instructions.push(Code::BR(TEMP));
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
        instructions.push(Code::SDIV(
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
        instructions.push(Code::SDIV(TEMP, source_temporary_1, source_temporary_2));
        instructions.push(Code::MSUB(
            target_temporary,
            TEMP,
            source_temporary_2,
            source_temporary_1,
        ));
    }

    fn mov(
        &self,
        target_temporary: Register,
        source_temporary: Register,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::MOVR(target_temporary, source_temporary));
    }
}
