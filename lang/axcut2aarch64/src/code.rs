use super::config::{Immediate, Register, TEMP};
use super::Backend;

use axcut::syntax::Name;
use axcut2backend::code::Instructions;
use printer::theme::ThemeExt;
use printer::tokens::{COLON, COMMA};
use printer::{DocAllocator, Print};

#[allow(non_camel_case_types)]
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
    MOVZ(Register, Immediate, Immediate),
    MOVN(Register, Immediate, Immediate),
    MOVK(Register, Immediate, Immediate),
    LDR(Register, Register, Immediate),
    /// This instruction is only used in the cleanup code.
    LDR_POST_INDEX(Register, Register, Immediate),
    STR(Register, Register, Immediate),
    /// This instruction is only used in the setup code.
    STR_PRE_INDEX(Register, Register, Immediate),
    CMPR(Register, Register),
    CMPI(Register, Immediate),
    BEQ(String),
    BLT(String),
    LAB(String),
    RET,
    GLOBAL(String),
    TEXT,
    COMMENT(String),
}

pub struct Codes {
    pub instructions: Vec<Code>,
}

impl Print for Codes {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let decls = self.instructions.iter().map(|decl| decl.print(cfg, alloc));
        alloc.intersperse(decls, alloc.line())
    }
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
                .append(z.print(cfg, alloc)),
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
                .append(z.print(cfg, alloc)),
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
                .keyword("MOV")
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
                .append(i.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(alloc.keyword("LSL"))
                .append(alloc.space())
                .append(s.print(cfg, alloc)),
            MOVN(register, i, s) => alloc
                .keyword("MOVN")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(alloc.keyword("LSL"))
                .append(alloc.space())
                .append(s.print(cfg, alloc)),
            MOVK(register, i, s) => alloc
                .keyword("MOVK")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(alloc.keyword("LSL"))
                .append(alloc.space())
                .append(s.print(cfg, alloc)),
            LDR(register, register1, i) => alloc
                .keyword("LDR")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(alloc.space())
                .append(register1.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append(alloc.space())
                .append("]"),
            LDR_POST_INDEX(register, register1, i) => alloc
                .keyword("LDR")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(alloc.space())
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc)),
            STR(register, register1, i) => alloc
                .keyword("STR")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(alloc.space())
                .append(register1.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append(alloc.space())
                .append("]"),
            STR_PRE_INDEX(register, register1, i) => alloc
                .keyword("STR")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(alloc.space())
                .append(register1.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append(alloc.space())
                .append("]!"),
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
                .append(i.print(cfg, alloc)),
            BEQ(l) => alloc.keyword("BEQ").append(alloc.space()).append(l),
            BLT(l) => alloc.keyword("BLT").append(alloc.space()).append(l),
            LAB(l) => alloc.hardline().append(l).append(COLON),
            RET => alloc.keyword("RET"),
            GLOBAL(l) => alloc.keyword(".global").append(alloc.space()).append(l),
            TEXT => alloc.keyword(".text"),
            COMMENT(msg) => alloc.comment(&format!("// {msg}")),
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
        instructions.push(Code::BR(temporary));
    }

    fn jump_label(name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::B(name));
    }

    fn jump_label_if_equal(fst: Register, snd: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::CMPR(fst, snd));
        instructions.push(Code::BEQ(name));
    }

    fn jump_label_if_less(fst: Register, snd: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::CMPR(fst, snd));
        instructions.push(Code::BLT(name));
    }

    fn jump_label_if_zero(temporary: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::CMPI(temporary, 0.into()));
        instructions.push(Code::BEQ(name));
    }

    fn load_immediate(temporary: Register, immediate: Immediate, instructions: &mut Vec<Code>) {
        fn number_unset_halfwords(immediate: Immediate) -> usize {
            let mut unset_halfwords = 0;
            for i in 0..4 {
                if (immediate.val >> (i * 16)).trailing_zeros() >= 16 {
                    unset_halfwords += 1;
                }
            }
            unset_halfwords
        }

        // the cases where all bits are 0 or all bits are 1 are special
        // we could further special-case immediates that can be expressed as bitmask-immediates
        // (using ORR)
        if immediate.val == 0 {
            instructions.push(Code::MOVZ(temporary, 0.into(), 0.into()));
        } else if immediate.val == -1 {
            instructions.push(Code::MOVN(temporary, 0.into(), 0.into()));
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
                let halfword = ((immediate.val >> shift) & 0xFFFF) as u16;
                if halfword != ignored_halfword {
                    if first_move_done {
                        instructions.push(Code::MOVK(
                            temporary,
                            i64::from(halfword).into(),
                            shift.into(),
                        ));
                    } else {
                        if invert {
                            instructions.push(Code::MOVN(
                                temporary,
                                i64::from(!halfword).into(),
                                shift.into(),
                            ));
                        } else {
                            instructions.push(Code::MOVZ(
                                temporary,
                                i64::from(halfword).into(),
                                shift.into(),
                            ));
                        }
                        first_move_done = true;
                    }
                }
            }
        }
    }

    fn load_label(temporary: Register, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::ADR(temporary, name));
    }

    fn add_and_jump(temporary: Register, immediate: Immediate, instructions: &mut Vec<Code>) {
        instructions.push(Code::ADDI(TEMP, temporary, immediate));
        instructions.push(Code::BR(TEMP));
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
        instructions.push(Code::SDIV(
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
        instructions.push(Code::SDIV(TEMP, source_temporary_1, source_temporary_2));
        instructions.push(Code::MSUB(
            target_temporary,
            TEMP,
            source_temporary_2,
            source_temporary_1,
        ));
    }

    fn mov(target_temporary: Register, source_temporary: Register, instructions: &mut Vec<Code>) {
        instructions.push(Code::MOVR(target_temporary, source_temporary));
    }
}
