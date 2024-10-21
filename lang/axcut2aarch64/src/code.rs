use super::config::{Immediate, Register};

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

#[must_use]
pub fn pretty(instructions: Vec<Code>) -> String {
    instructions
        .into_iter()
        .map(|code| format!("{code}"))
        .collect::<Vec<String>>()
        .join("\n")
}
