use super::config::{
    address, Immediate, Register, CALLER_SAVE_FIRST, CALLER_SAVE_LAST, REGISTER_NUM, RESERVED, TEMP,
};
use super::Backend;

use axcut::syntax::{Chirality, ContextBinding, Name};
use axcut2backend::code::Instructions;
use printer::theme::ThemeExt;
use printer::tokens::{COLON, COMMA, PRINTLN_I64};
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
    BL(String),
    ADR(Register, String),
    MOVR(Register, Register),
    MOVZ(Register, Immediate, Immediate),
    MOVN(Register, Immediate, Immediate),
    MOVK(Register, Immediate, Immediate),
    LDR(Register, Register, Immediate),
    /// This instruction is only used in the cleanup code.
    LDP_POST_INDEX(Register, Register, Register, Immediate),
    STR(Register, Register, Immediate),
    /// This instruction is only used in the setup code.
    STP_PRE_INDEX(Register, Register, Register, Immediate),
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
        const INDENT: &str = "    ";
        use Code::*;
        match self {
            ADD(x, y, z) => alloc
                .text(INDENT)
                .append(alloc.keyword("ADD"))
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            ADDI(x, y, z) => alloc
                .text(INDENT)
                .append(alloc.keyword("ADD"))
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            SUB(x, y, z) => alloc
                .text(INDENT)
                .append(alloc.keyword("SUB"))
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            SUBI(x, y, z) => alloc
                .text(INDENT)
                .append(alloc.keyword("SUB"))
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            MUL(x, y, z) => alloc
                .text(INDENT)
                .append(alloc.keyword("MUL"))
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            SDIV(x, y, z) => alloc
                .text(INDENT)
                .append(alloc.keyword("SDIV"))
                .append(alloc.space())
                .append(x.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(y.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(z.print(cfg, alloc)),
            MSUB(x, y, z, v) => alloc
                .text(INDENT)
                .append(alloc.keyword("MSUB"))
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
            B(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("B"))
                .append(alloc.space())
                .append(l),
            BR(r) => alloc
                .text(INDENT)
                .append(alloc.keyword("BR"))
                .append(alloc.space())
                .append(r.print(cfg, alloc)),
            BL(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("BL"))
                .append(alloc.space())
                .append(l),
            ADR(register, l) => alloc
                .text(INDENT)
                .append(alloc.keyword("ADR"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(l),
            MOVR(register, register1) => alloc
                .text(INDENT)
                .append(alloc.keyword("MOV"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            MOVZ(register, i, s) => alloc
                .text(INDENT)
                .append(alloc.keyword("MOVZ"))
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
                .text(INDENT)
                .append(alloc.keyword("MOVN"))
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
                .text(INDENT)
                .append(alloc.keyword("MOVK"))
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
                .text(INDENT)
                .append(alloc.keyword("LDR"))
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
            LDP_POST_INDEX(register1, register2, register, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("LDP"))
                .append(alloc.space())
                .append(register1.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register2.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc)),
            STR(register, register1, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("STR"))
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
            STP_PRE_INDEX(register1, register2, register, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("STP"))
                .append(alloc.space())
                .append(register1.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register2.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append(alloc.space())
                .append("]!"),
            CMPR(register, register1) => alloc
                .text(INDENT)
                .append(alloc.keyword("CMP"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            CMPI(register, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("CMP"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc)),
            BEQ(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("BEQ"))
                .append(alloc.space())
                .append(l),
            BLT(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("BLT"))
                .append(alloc.space())
                .append(l),
            LAB(l) => alloc.hardline().append(l).append(COLON),
            RET => alloc.text(INDENT).append(alloc.keyword("RET")),
            GLOBAL(l) => alloc.keyword(".global").append(alloc.space()).append(l),
            TEXT => alloc.keyword(".text"),
            COMMENT(msg) => alloc
                .text(INDENT)
                .append(alloc.comment(&format!("// {msg}"))),
        }
    }
}

fn caller_save_registers_info(context: &[ContextBinding]) -> (usize, Vec<usize>) {
    let first_free_register = 2 * context.len() + RESERVED;
    let first_backup_register = std::cmp::max(first_free_register, CALLER_SAVE_LAST + 1);

    let caller_save_count = CALLER_SAVE_LAST + 1 - CALLER_SAVE_FIRST;
    let mut registers_to_save = Vec::with_capacity(caller_save_count + 3);
    // we always have to save the first two registers, containing `HEAP` and `FREE`
    registers_to_save.push(0);
    registers_to_save.push(1);
    // the last register will contain the return address, so it must be saved if in use
    if first_free_register > REGISTER_NUM {
        registers_to_save.push(REGISTER_NUM - 1);
    }
    // if all caller-save registers are in use, the last of them will only contain data to save if
    // the variable it belongs to is not of extern type
    let caller_save_var_count = (caller_save_count - 1) / 2;
    if first_free_register > CALLER_SAVE_LAST
        && (context[caller_save_var_count].chi != Chirality::Ext)
    {
        registers_to_save.push(CALLER_SAVE_LAST);
    }
    for (offset, binding) in context.iter().take(caller_save_var_count).enumerate() {
        if binding.chi == Chirality::Ext {
            registers_to_save.push(CALLER_SAVE_FIRST + 2 * offset + 1);
        } else {
            registers_to_save.push(CALLER_SAVE_FIRST + 2 * offset);
            registers_to_save.push(CALLER_SAVE_FIRST + 2 * offset + 1);
        }
    }

    (first_backup_register, registers_to_save)
}

fn save_caller_save_registers(
    first_backup_register: usize,
    registers_to_save: &[usize],
    instructions: &mut Vec<Code>,
) {
    let registers_to_save_count = registers_to_save.len();
    // the last register will contain the return address
    let backup_register_count = std::cmp::max(REGISTER_NUM - 1 - first_backup_register, 0);
    let backup_registers_used = std::cmp::min(registers_to_save_count, backup_register_count);

    for (offset, register) in registers_to_save
        .iter()
        .take(backup_registers_used)
        .enumerate()
    {
        instructions.push(Code::MOVR(
            (first_backup_register + offset).into(),
            (*register).into(),
        ));
    }

    let mut registers_to_push_count = registers_to_save_count - backup_registers_used;
    if registers_to_push_count > 0 {
        // ensure stack pointer alignment
        if registers_to_push_count % 2 == 0 {
            registers_to_push_count += 1;
        }
        instructions.push(Code::SUBI(
            Register::SP,
            Register::SP,
            address(registers_to_push_count as i64).into(),
        ));
        for (offset, register) in registers_to_save
            .iter()
            .skip(backup_registers_used)
            .enumerate()
        {
            instructions.push(Code::STR(
                (*register).into(),
                Register::SP,
                address((registers_to_push_count - 1 - offset) as i64).into(),
            ));
        }
    }
}

fn restore_caller_save_registers(
    first_backup_register: usize,
    registers_to_save: &[usize],
    instructions: &mut Vec<Code>,
) {
    let registers_to_save_count = registers_to_save.len();
    // the last register will contain the return address
    let backup_register_count = std::cmp::max(REGISTER_NUM - 1 - first_backup_register, 0);
    let backup_registers_used = std::cmp::min(registers_to_save_count, backup_register_count);

    for (offset, register) in registers_to_save
        .iter()
        .take(backup_registers_used)
        .enumerate()
    {
        instructions.push(Code::MOVR(
            (*register).into(),
            (first_backup_register + offset).into(),
        ));
    }

    let mut registers_to_push_count = registers_to_save_count - backup_registers_used;
    if registers_to_push_count > 0 {
        // ensure stack pointer alignment
        if registers_to_push_count % 2 == 0 {
            registers_to_push_count += 1;
        }
        for (offset, register) in registers_to_save
            .iter()
            .skip(backup_registers_used)
            .enumerate()
            .rev()
        {
            instructions.push(Code::LDR(
                (*register).into(),
                Register::SP,
                address((registers_to_push_count - 1 - offset) as i64).into(),
            ));
        }
        instructions.push(Code::ADDI(
            Register::SP,
            Register::SP,
            address(registers_to_push_count as i64).into(),
        ));
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

    fn jump_label_fixed(name: Name, instructions: &mut Vec<Code>) {
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

    fn println_i64(
        source_temporary: Register,
        context: &[ContextBinding],
        instructions: &mut Vec<Code>,
    ) {
        let (first_backup_register, registers_to_save) = caller_save_registers_info(context);

        instructions.push(Code::COMMENT("#save caller-save registers".to_string()));
        save_caller_save_registers(first_backup_register, &registers_to_save, instructions);
        instructions.push(Code::COMMENT("#move argument into place".to_string()));
        instructions.push(Code::MOVR(Register::X(0), source_temporary));
        instructions.push(Code::BL(PRINTLN_I64.to_string()));
        instructions.push(Code::COMMENT("#restore caller-save registers".to_string()));
        restore_caller_save_registers(first_backup_register, &registers_to_save, instructions);
    }
}
