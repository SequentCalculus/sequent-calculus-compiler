use super::config::{
    address, stack_offset, Immediate, Register, Temporary, CALLER_SAVE_FIRST, CALLER_SAVE_LAST,
    REGISTER_NUM, RESERVED, SPILL_TEMP, TEMP, TEMP2, TEMPORARY_TEMP,
};
use super::Backend;

use axcut::syntax::{Chirality, ContextBinding, Name};
use axcut2backend::code::Instructions;
use printer::theme::ThemeExt;
use printer::tokens::{COLON, COMMA, PRINTLN_I64, PRINT_I64};
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
    BNE(String),
    BLT(String),
    BLE(String),
    LAB(String),
    RET,
    GLOBAL(String),
    TEXT,
    COMMENT(String),
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
            BNE(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("BNE"))
                .append(alloc.space())
                .append(l),
            BLT(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("BLT"))
                .append(alloc.space())
                .append(l),
            BLE(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("BLE"))
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

pub fn move_from_register(temporary: Temporary, register: Register, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(target_register) => {
            instructions.push(Code::MOVR(target_register, register));
        }
        Temporary::Spill(target_position) => {
            instructions.push(Code::STR(
                register,
                Register::SP,
                stack_offset(target_position),
            ));
        }
    }
}

pub fn move_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::MOVR(register, source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::LDR(
                register,
                Register::SP,
                stack_offset(source_position),
            ));
        }
    }
}

pub fn add(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::ADD(target, source_1, source_2));
}

pub fn sub(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::SUB(target, source_1, source_2));
}

pub fn mul(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::MUL(target, source_1, source_2));
}

pub fn div(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::SDIV(target, source_1, source_2));
}

pub fn rem(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
    // this also means `source_1 == TEMP`
    if source_2 == TEMP2 {
        if target == TEMP {
            instructions.push(Code::COMMENT(
                "#evacuate one register as additional scratch register".to_string(),
            ));
            instructions.push(Code::STR(
                TEMPORARY_TEMP,
                Register::SP,
                stack_offset(SPILL_TEMP),
            ));
            instructions.push(Code::MOVR(TEMPORARY_TEMP, source_2));
            instructions.push(Code::SDIV(TEMP2, source_1, TEMPORARY_TEMP));
            instructions.push(Code::MSUB(target, TEMP2, TEMPORARY_TEMP, source_1));
            instructions.push(Code::COMMENT("#restore evacuated register".to_string()));
            instructions.push(Code::LDR(
                TEMPORARY_TEMP,
                Register::SP,
                stack_offset(SPILL_TEMP),
            ));
        } else {
            instructions.push(Code::SDIV(target, source_1, source_2));
            instructions.push(Code::MSUB(target, target, source_2, source_1));
        }
    } else {
        instructions.push(Code::SDIV(TEMP2, source_1, source_2));
        instructions.push(Code::MSUB(target, TEMP2, source_2, source_1));
    }
}

pub fn op(
    op: fn(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>),
    target_temporary: Temporary,
    source_temporary_1: Temporary,
    source_temporary_2: Temporary,
    instructions: &mut Vec<Code>,
) {
    match target_temporary {
        Temporary::Register(target_register) => match (source_temporary_1, source_temporary_2) {
            (Temporary::Register(source_register_1), Temporary::Register(source_register_2)) => {
                op(
                    target_register,
                    source_register_1,
                    source_register_2,
                    instructions,
                );
            }
            (Temporary::Register(source_register_1), Temporary::Spill(source_position_2)) => {
                instructions.push(Code::LDR(
                    TEMP,
                    Register::SP,
                    stack_offset(source_position_2),
                ));
                op(target_register, source_register_1, TEMP, instructions);
            }
            (Temporary::Spill(source_position_1), Temporary::Register(source_register_2)) => {
                instructions.push(Code::LDR(
                    TEMP,
                    Register::SP,
                    stack_offset(source_position_1),
                ));
                op(target_register, TEMP, source_register_2, instructions);
            }
            (Temporary::Spill(source_position_1), Temporary::Spill(source_position_2)) => {
                instructions.push(Code::LDR(
                    TEMP,
                    Register::SP,
                    stack_offset(source_position_1),
                ));
                instructions.push(Code::LDR(
                    TEMP2,
                    Register::SP,
                    stack_offset(source_position_2),
                ));
                op(target_register, TEMP, TEMP2, instructions);
            }
        },
        Temporary::Spill(target_position) => {
            match (source_temporary_1, source_temporary_2) {
                (
                    Temporary::Register(source_register_1),
                    Temporary::Register(source_register_2),
                ) => {
                    op(TEMP, source_register_1, source_register_2, instructions);
                }
                (Temporary::Register(source_register_1), Temporary::Spill(source_position_2)) => {
                    instructions.push(Code::LDR(
                        TEMP,
                        Register::SP,
                        stack_offset(source_position_2),
                    ));
                    op(TEMP, source_register_1, TEMP, instructions);
                }
                (Temporary::Spill(source_position_1), Temporary::Register(source_register_2)) => {
                    instructions.push(Code::LDR(
                        TEMP,
                        Register::SP,
                        stack_offset(source_position_1),
                    ));
                    op(TEMP, TEMP, source_register_2, instructions);
                }
                (Temporary::Spill(source_position_1), Temporary::Spill(source_position_2)) => {
                    instructions.push(Code::LDR(
                        TEMP,
                        Register::SP,
                        stack_offset(source_position_1),
                    ));
                    instructions.push(Code::LDR(
                        TEMP2,
                        Register::SP,
                        stack_offset(source_position_2),
                    ));
                    op(TEMP, TEMP, TEMP2, instructions);
                }
            }
            instructions.push(Code::STR(TEMP, Register::SP, stack_offset(target_position)));
        }
    }
}

pub fn compare(fst: Temporary, snd: Temporary, instructions: &mut Vec<Code>) {
    match (fst, snd) {
        (Temporary::Register(register_fst), Temporary::Register(register_snd)) => {
            instructions.push(Code::CMPR(register_fst, register_snd));
        }
        (Temporary::Register(register_fst), Temporary::Spill(position_snd)) => {
            instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position_snd)));
            instructions.push(Code::CMPR(register_fst, TEMP));
        }
        (Temporary::Spill(position_fst), Temporary::Register(register_snd)) => {
            instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position_fst)));
            instructions.push(Code::CMPR(TEMP, register_snd));
        }
        (Temporary::Spill(position_fst), Temporary::Spill(position_snd)) => {
            instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position_fst)));
            instructions.push(Code::LDR(TEMP2, Register::SP, stack_offset(position_snd)));
            instructions.push(Code::CMPR(TEMP, TEMP2));
        }
    }
}

pub fn compare_immediate(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(register) => instructions.push(Code::CMPI(register, immediate)),
        Temporary::Spill(position) => {
            instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position)));
            instructions.push(Code::CMPI(TEMP, immediate));
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
    for (offset, binding) in context.iter().take(caller_save_count / 2).enumerate() {
        if binding.chi == Chirality::Ext {
            registers_to_save.push(CALLER_SAVE_FIRST + 2 * offset + 1);
        } else {
            registers_to_save.push(CALLER_SAVE_FIRST + 2 * offset);
            registers_to_save.push(CALLER_SAVE_FIRST + 2 * offset + 1);
        }
    }

    (first_backup_register, registers_to_save)
}

#[allow(clippy::cast_possible_wrap)]
fn save_caller_save_registers(
    first_backup_register: usize,
    registers_to_save: &[usize],
    instructions: &mut Vec<Code>,
) {
    let registers_to_save_count = registers_to_save.len();
    // the last register will contain the return address
    let backup_register_count = if REGISTER_NUM - 1 < first_backup_register {
        0
    } else {
        REGISTER_NUM - 1 - first_backup_register
    };
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
        if registers_to_push_count % 2 != 0 {
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

#[allow(clippy::cast_possible_wrap)]
fn restore_caller_save_registers(
    first_backup_register: usize,
    registers_to_save: &[usize],
    instructions: &mut Vec<Code>,
) {
    let registers_to_save_count = registers_to_save.len();
    // the last register will contain the return address
    let backup_register_count = if REGISTER_NUM - 1 < first_backup_register {
        0
    } else {
        REGISTER_NUM - 1 - first_backup_register
    };
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
        if registers_to_push_count % 2 != 0 {
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

impl Instructions<Code, Temporary, Immediate> for Backend {
    fn comment(msg: String) -> Code {
        Code::COMMENT(msg)
    }

    fn label(name: Name) -> Code {
        Code::LAB(name)
    }

    fn jump(temporary: Temporary, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::BR(register)),
            Temporary::Spill(position) => {
                instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position)));
                instructions.push(Code::BR(TEMP));
            }
        }
    }

    fn jump_label(name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::B(name));
    }

    fn jump_label_fixed(name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::B(name));
    }

    fn jump_label_if_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BEQ(name));
    }

    fn jump_label_if_not_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BNE(name));
    }

    fn jump_label_if_less(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BLT(name));
    }

    fn jump_label_if_less_or_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BLE(name));
    }

    fn jump_label_if_zero(temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::BEQ(name));
    }

    fn jump_label_if_not_zero(temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::BNE(name));
    }

    #[allow(clippy::cast_sign_loss)]
    fn load_immediate(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
        fn number_unset_halfwords(immediate: Immediate) -> usize {
            let mut unset_halfwords = 0;
            for i in 0..4 {
                if (immediate.val >> (i * 16)).trailing_zeros() >= 16 {
                    unset_halfwords += 1;
                }
            }
            unset_halfwords
        }

        let register = match temporary {
            Temporary::Register(register) => register,
            Temporary::Spill(_) => TEMP,
        };

        // the cases where all bits are 0 or all bits are 1 are special
        // we could further special-case immediates that can be expressed as bitmask-immediates
        // (using ORR)
        if immediate.val == 0 {
            instructions.push(Code::MOVZ(register, 0.into(), 0.into()));
        } else if immediate.val == -1 {
            instructions.push(Code::MOVN(register, 0.into(), 0.into()));
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
                            register,
                            i64::from(halfword).into(),
                            shift.into(),
                        ));
                    } else {
                        if invert {
                            instructions.push(Code::MOVN(
                                register,
                                i64::from(!halfword).into(),
                                shift.into(),
                            ));
                        } else {
                            instructions.push(Code::MOVZ(
                                register,
                                i64::from(halfword).into(),
                                shift.into(),
                            ));
                        }
                        first_move_done = true;
                    }
                }
            }
        }

        match temporary {
            Temporary::Register(_) => {}
            Temporary::Spill(position) => {
                instructions.push(Code::STR(TEMP, Register::SP, stack_offset(position)));
            }
        }
    }

    fn load_label(temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::ADR(register, name)),
            Temporary::Spill(position) => {
                instructions.push(Code::ADR(TEMP, name));
                instructions.push(Code::STR(TEMP, Register::SP, stack_offset(position)));
            }
        }
    }

    fn add_and_jump(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => {
                instructions.push(Code::ADDI(register, register, immediate));
                instructions.push(Code::BR(register));
            }
            Temporary::Spill(position) => {
                instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position)));
                instructions.push(Code::ADDI(TEMP, TEMP, immediate));
                instructions.push(Code::BR(TEMP));
            }
        }
    }

    fn add(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        op(
            add,
            target_temporary,
            source_temporary_1,
            source_temporary_2,
            instructions,
        );
    }

    fn sub(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        op(
            sub,
            target_temporary,
            source_temporary_1,
            source_temporary_2,
            instructions,
        );
    }

    fn mul(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        op(
            mul,
            target_temporary,
            source_temporary_1,
            source_temporary_2,
            instructions,
        );
    }

    fn div(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        op(
            div,
            target_temporary,
            source_temporary_1,
            source_temporary_2,
            instructions,
        );
    }

    fn rem(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        op(
            rem,
            target_temporary,
            source_temporary_1,
            source_temporary_2,
            instructions,
        );
    }

    fn mov(target_temporary: Temporary, source_temporary: Temporary, instructions: &mut Vec<Code>) {
        if let Temporary::Register(source_register) = source_temporary {
            move_from_register(target_temporary, source_register, instructions);
        } else if let Temporary::Register(target_register) = target_temporary {
            move_to_register(target_register, source_temporary, instructions);
        } else {
            move_to_register(TEMP, source_temporary, instructions);
            move_from_register(target_temporary, TEMP, instructions);
        }
    }

    fn print_i64(
        newline: bool,
        source_temporary: Temporary,
        context: &[ContextBinding],
        instructions: &mut Vec<Code>,
    ) {
        let print_i64 = if newline { PRINTLN_I64 } else { PRINT_I64 };
        let (first_backup_register, registers_to_save) = caller_save_registers_info(context);

        // alternatively, we could take the change of the stack pointer into consideration when
        // moving the argument into place
        if let Temporary::Spill(_) = source_temporary {
            instructions.push(Code::COMMENT(
                "#move argument to TEMP before adapting the stack pointer".to_string(),
            ));
            move_to_register(TEMP, source_temporary, instructions);
        }

        instructions.push(Code::COMMENT("#save caller-save registers".to_string()));
        save_caller_save_registers(first_backup_register, &registers_to_save, instructions);
        instructions.push(Code::COMMENT("#move argument into place".to_string()));
        match source_temporary {
            Temporary::Register(source_register) => {
                instructions.push(Code::MOVR(Register::X(0), source_register))
            }
            Temporary::Spill(_) => instructions.push(Code::MOVR(Register::X(0), TEMP)),
        }
        instructions.push(Code::BL(print_i64.to_string()));
        instructions.push(Code::COMMENT("#restore caller-save registers".to_string()));
        restore_caller_save_registers(first_backup_register, &registers_to_save, instructions);
    }
}
