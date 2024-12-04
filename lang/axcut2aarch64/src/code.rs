use super::config::{Immediate, Register, TEMP};
use super::Backend;

use axcut::syntax::Name;
use axcut2backend::code::Instructions;
use printer::theme::ThemeExt;
use printer::tokens::{COLON, COMMA};
use printer::{DocAllocator, Print};

use std::fmt;

#[derive(Debug, Clone)]
pub enum Code {
    ADD(Register, Register, Register),
    ADDI(Register, Register, Immediate),
    SUB(Register, Register, Register),
    SUBI(Register, Register, Immediate),
    MUL(Register, Register, Register),
    SDIV(Register, Register, Register),
    MSUB(Register, Register, Register, Register),
    B(String),
    BR(Register),
    ADR(Register, String),
    MOVR(Register, Register),
    MOVZ(Register, u16, u8),
    MOVN(Register, u16, u8),
    MOVK(Register, u16, u8),
    LDR(Register, Register, Immediate),
    STR(Register, Register, Immediate),
    CMPR(Register, Register),
    CMPI(Register, Immediate),
    BEQ(String),
    BLT(String),
    LAB(String),
    COMMENT(String),
}

impl Print for Code {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        use Code::*;
        match self {
            ADD(x, y, z) => alloc
                .keyword("ADD")
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            ADDI(x, y, z) => alloc
                .keyword("ADD")
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{}", z)),
            SUB(x, y, z) => alloc
                .keyword("SUB")
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            SUBI(x, y, z) => alloc
                .keyword("SUB")
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{}", z)),
            MUL(x, y, z) => alloc
                .keyword("MUL")
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            SDIV(x, y, z) => alloc
                .keyword("SDIV")
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            MSUB(x, y, z, v) => alloc
                .keyword("MSUB")
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(v.print(cfg, alloc)),
            B(l) => alloc.keyword("B").append(alloc.space()).append(l),
            BR(r) => alloc
                .keyword("BR")
                .append(alloc.space())
                .append(r.print(cfg, alloc)),
            ADR(register, l) => alloc
                .keyword("ADR")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(l),
            MOVR(register, register1) => alloc
                .keyword("MOVR")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            MOVZ(register, i, s) => alloc
                .keyword("MOVZ")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{}", i))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("LSL {}", s)),
            MOVN(register, i, s) => alloc
                .keyword("MOVN")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{}", i))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("LSL {}", s)),
            MOVK(register, i, s) => alloc
                .keyword("MOVR")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{}", i))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("LSL {}", s)),
            LDR(register, register1, i) => alloc
                .keyword("LDR")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{}", i))
                .append("]"),
            STR(register, register1, i) => alloc
                .keyword("STR")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{}", i))
                .append("]"),
            CMPR(register, register1) => alloc
                .keyword("CMP")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            CMPI(register, i) => alloc
                .keyword("CMP")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{}", i)),
            BEQ(l) => alloc.keyword("BEQ").append(alloc.space()).append(l),
            BLT(l) => alloc.keyword("BLT").append(alloc.space()).append(l),
            LAB(l) => alloc.hardline().append(l).append(COLON),
            COMMENT(msg) => alloc.comment(&format!("// {msg}")),
        }
    }
}
impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Code::*;
        match self {
            ADD(x, y, z) => write!(f, "ADD {x}, {y}, {z}"),
            ADDI(x, y, c) => write!(f, "ADD {x}, {y}, {c}"),
            SUB(x, y, z) => write!(f, "SUB {x}, {y}, {z}"),
            SUBI(x, y, c) => write!(f, "SUB {x}, {y}, {c}"),
            MUL(x, y, z) => write!(f, "MUL {x}, {y}, {z}"),
            SDIV(x, y, z) => write!(f, "SDIV {x}, {y}, {z}"),
            MSUB(x, y, z, v) => write!(f, "MSUB {x}, {y}, {z}, {v}"),
            B(l) => write!(f, "B {l}"),
            BR(x) => write!(f, "BR {x}"),
            ADR(x, l) => write!(f, "ADR {x}, {l}"),
            MOVR(x, y) => write!(f, "MOV {x}, {y}"),
            MOVZ(x, c, s) => write!(f, "MOVZ {x}, {c}, LSL {s}"),
            MOVN(x, c, s) => write!(f, "MOVN {x}, {c}, LSL {s}"),
            MOVK(x, c, s) => write!(f, "MOVK {x}, {c}, LSL {s}"),
            LDR(x, y, c) => write!(f, "LDR {x}, [ {y}, {c} ]"),
            STR(x, y, c) => write!(f, "STR {x}, [ {y}, {c} ]"),
            CMPR(x, y) => write!(f, "CMP {x}, {y}"),
            CMPI(x, c) => write!(f, "CMP {x}, {c}"),
            BEQ(l) => write!(f, "BEQ {l}"),
            BLT(l) => write!(f, "BLT {l}"),
            LAB(l) => write!(f, "\n{l}:"),
            COMMENT(msg) => write!(f, "// {msg}"),
        }
    }
}

impl Instructions<Code, Register, Immediate> for Backend {
    fn comment(&self, msg: String) -> Code {
        Code::COMMENT(msg)
    }

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
        fn number_unset_halfwords(immediate: Immediate) -> usize {
            let mut unset_halfwords = 0;
            for i in 0..4 {
                if (immediate >> (i * 16)) & 0xFFFF == 0 {
                    unset_halfwords += 1
                }
            }
            unset_halfwords
        }

        // the cases where all bits are 0 or all bits are 1 are special
        // we could further special-case immediates that can be expressed as bitmask-immediates
        // (using ORR)
        if immediate == 0 {
            instructions.push(Code::MOVZ(temporary, 0, 0));
        } else if immediate == -1 {
            instructions.push(Code::MOVN(temporary, 0, 0));
        } else {
            // otherwise, we consider the four halfwords separately
            // we move the first non-ignored halfword with MOVZ or MOVN and the other ones with MOVK

            // if there are more 0xFFFF halfwords than 0x0000 halfwords, then it is more efficient to
            // ignore 0xFFFF the former and bit-wise invert (MOVN) the first non-ignored halfword
            let (invert, ignored_halfword) =
                if number_unset_halfwords(immediate) < number_unset_halfwords(!immediate) {
                    (true, 0xFFFF)
                } else {
                    (false, 0)
                };

            let mut first_move_done = false;
            // iterate through the halfwords
            for i in 0..4 {
                let shift = i * 16;
                let halfword = ((immediate >> shift) & 0xFFFF) as u16;
                if halfword != ignored_halfword {
                    if !first_move_done {
                        if invert {
                            instructions.push(Code::MOVN(temporary, !halfword, shift));
                        } else {
                            instructions.push(Code::MOVZ(temporary, halfword, shift));
                        }
                        first_move_done = true;
                    } else {
                        instructions.push(Code::MOVK(temporary, halfword, shift));
                    }
                }
            }
        }
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
