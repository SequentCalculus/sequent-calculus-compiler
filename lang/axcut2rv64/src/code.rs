//! This module implements the abstract functions for machine instructions.

use super::Backend;
use super::config::{Immediate, Register, TEMP, ZERO};

use axcut::syntax::{ContextBinding, Name};
use axcut2backend::code::Instructions;

use std::fmt;

/// This enum provides the concrete machine instructions. Each variant stands either for one
/// instruction or pseudo-instruction or for a label or comment. However, this currently does not
/// take into account that [`super::config::Immediate`]s are restricted in size, as all
/// instructions are 32 bits long.
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
    BNE(Register, Register, String),
    BLT(Register, Register, String),
    BLE(Register, Register, String),
    LAB(String),
    COMMENT(String),
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Code::*;
        match self {
            ADD(x, y, z) => write!(f, "ADD {x} {y} {z}"),
            ADDI(x, y, c) => write!(f, "ADD {x} {y} {c}"),
            SUB(x, y, z) => write!(f, "SUB {x} {y} {z}"),
            MUL(x, y, z) => write!(f, "MUL {x} {y} {z}"),
            DIV(x, y, z) => write!(f, "MUL {x} {y} {z}"),
            REM(x, y, z) => write!(f, "REM {x} {y} {z}"),
            JAL(x, l) => write!(f, "JAL {x} {l}"),
            JALR(x, y, c) => write!(f, "JALR {x} {y} {c}"),
            LA(x, l) => write!(f, "LA {x} {l}"),
            LI(x, c) => write!(f, "LI {x} {c}"),
            MV(x, y) => write!(f, "MV {x} {y}"),
            LW(x, y, c) => write!(f, "LW {x} {c} {y}"),
            SW(x, y, c) => write!(f, "SW {x} {c} {y}"),
            BEQ(x, y, l) => write!(f, "BEQ {x} {y} {l}"),
            BNE(x, y, l) => write!(f, "BNE {x} {y} {l}"),
            BLT(x, y, l) => write!(f, "BLT {x} {y} {l}"),
            BLE(x, y, l) => write!(f, "BLE {x} {y} {l}"),
            LAB(l) => write!(f, "\n{l}:"),
            COMMENT(msg) => write!(f, "// {msg}"),
        }
    }
}

impl Instructions<Code, Register, Immediate> for Backend {
    fn comment(msg: String) -> Code {
        Code::COMMENT(msg)
    }

    fn label(name: Name) -> Code {
        Code::LAB(name)
    }

    fn jump(temporary: Register, instructions: &mut Vec<Code>) {
        instructions.push(Code::JALR(ZERO, temporary, 0));
    }

    fn jump_label(name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::JAL(ZERO, name));
    }

    fn jump_label_fixed(name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::JAL(ZERO, name));
    }

    fn jump_label_if_equal(fst: Register, snd: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::BEQ(fst, snd, name));
    }

    fn jump_label_if_not_equal(
        fst: Register,
        snd: Register,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::BNE(fst, snd, name));
    }

    fn jump_label_if_less(fst: Register, snd: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::BLT(fst, snd, name));
    }

    fn jump_label_if_less_or_equal(
        fst: Register,
        snd: Register,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::BLE(fst, snd, name));
    }

    fn jump_label_if_zero(temporary: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::BEQ(temporary, ZERO, name));
    }

    fn jump_label_if_not_zero(temporary: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::BNE(temporary, ZERO, name));
    }

    fn load_immediate(temporary: Register, immediate: Immediate, instructions: &mut Vec<Code>) {
        instructions.push(Code::LI(temporary, immediate));
    }

    fn load_label(temporary: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::LA(temporary, name));
    }

    fn add_and_jump(temporary: Register, immediate: Immediate, instructions: &mut Vec<Code>) {
        instructions.push(Code::ADDI(TEMP, temporary, immediate));
        instructions.push(Code::JALR(ZERO, TEMP, 0));
    }

    fn add(
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

    fn mov(target_temporary: Register, source_temporary: Register, instructions: &mut Vec<Code>) {
        instructions.push(Code::MV(target_temporary, source_temporary));
    }

    fn print_i64(
        _newline: bool,
        _source_temporary: Register,
        _context: &[ContextBinding],
        _instructions: &mut Vec<Code>,
    ) {
        panic!("not implemented in RISC-V backend");
    }
}
