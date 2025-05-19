use super::Backend;
use super::config::{
    CALLER_SAVE_FIRST, CALLER_SAVE_LAST, Immediate, REGISTER_NUM, RESERVED, RETURN1, RETURN2,
    Register, STACK, Spill, TEMP, Temporary, address, arg, stack_offset,
};

use axcut::syntax::{Chirality, ContextBinding, Name};
use axcut2backend::code::Instructions;
use printer::theme::ThemeExt;
use printer::tokens::{COLON, COMMA, PLUS, PRINT_I64, PRINTLN_I64};
use printer::{DocAllocator, Print};

/// x86-64 Assembly instructions
#[derive(Debug, Clone)]
pub enum Code {
    /// https://www.felixcloutier.com/x86/add
    ADD(Register, Register),
    /// https://www.felixcloutier.com/x86/add
    ADDRM(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/add
    ADDMR(Register, Immediate, Register),
    /// https://www.felixcloutier.com/x86/add
    ADDI(Register, Immediate),
    /// https://www.felixcloutier.com/x86/add
    ADDIM(Register, Immediate, Immediate),
    /// https://www.felixcloutier.com/x86/sub
    SUB(Register, Register),
    /// https://www.felixcloutier.com/x86/sub
    SUBRM(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/sub
    SUBMR(Register, Immediate, Register),
    /// https://www.felixcloutier.com/x86/sub
    SUBI(Register, Immediate),
    /// https://www.felixcloutier.com/x86/imul
    IMUL(Register, Register),
    /// https://www.felixcloutier.com/x86/imul
    IMULRM(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/imul
    IMULMR(Register, Immediate, Register),
    /// https://www.felixcloutier.com/x86/idiv
    IDIV(Register),
    /// https://www.felixcloutier.com/x86/idiv
    IDIVM(Register, Immediate),
    /// https://www.felixcloutier.com/x86/cwd:cdq:cqo
    CQO,
    /// https://www.felixcloutier.com/x86/jmp
    JMP(Register),
    /// https://www.felixcloutier.com/x86/jmp
    JMPL(String),
    /// https://www.felixcloutier.com/x86/jmp
    JMPLN(String),
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
    /// https://www.felixcloutier.com/x86/jcc
    JEL(String),
    /// https://www.felixcloutier.com/x86/jcc
    JNEL(String),
    /// https://www.felixcloutier.com/x86/jcc
    JLL(String),
    /// https://www.felixcloutier.com/x86/jcc
    JLEL(String),
    PUSH(Register),
    POP(Register),
    CALL(String),
    RET,
    LAB(String),
    NOEXECSTACK,
    TEXT,
    GLOBAL(String),
    EXTERN(String),
    COMMENT(String),
}

impl Print for Code {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        use Code::*;
        const INDENT: &str = "    ";
        match self {
            ADD(register, register1) => alloc
                .text(INDENT)
                .append(alloc.keyword("add"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            ADDRM(register, register1, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("add"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]"),
            ADDMR(register1, i, register) => alloc
                .text(INDENT)
                .append(alloc.keyword("add"))
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(register.print(cfg, alloc)),
            ADDI(register, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("add"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc)),
            ADDIM(register, i1, i2) => alloc
                .text(INDENT)
                .append(alloc.keyword("add qword"))
                .append(alloc.space())
                .append("[")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i1.print(cfg, alloc))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(i2.print(cfg, alloc)),
            SUB(register, register1) => alloc
                .text(INDENT)
                .append(alloc.keyword("sub"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            SUBRM(register, register1, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("sub"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]"),
            SUBMR(register1, i, register) => alloc
                .text(INDENT)
                .append(alloc.keyword("sub"))
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(register.print(cfg, alloc)),
            SUBI(register, immediate) => alloc
                .text(INDENT)
                .append(alloc.keyword("sub"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(immediate.print(cfg, alloc)),
            IMUL(register, register1) => alloc
                .text(INDENT)
                .append(alloc.keyword("imul"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            IMULRM(register, register1, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("imul"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]"),
            IMULMR(register1, i, register) => alloc
                .text(INDENT)
                .append(alloc.keyword("imul"))
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(register.print(cfg, alloc)),
            IDIV(register) => alloc
                .text(INDENT)
                .append(alloc.keyword("idiv"))
                .append(alloc.space())
                .append(register.print(cfg, alloc)),
            IDIVM(register, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("idiv qword"))
                .append(alloc.space())
                .append("[")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]"),
            CQO => alloc.text(INDENT).append(alloc.keyword("cqo")),
            JMP(register) => alloc
                .text(INDENT)
                .append(alloc.keyword("jmp"))
                .append(alloc.space())
                .append(register.print(cfg, alloc)),
            JMPL(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("jmp"))
                .append(alloc.space())
                .append(l),
            JMPLN(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("jmp near"))
                .append(alloc.space())
                .append(l),
            LEAL(register, l) => alloc
                .text(INDENT)
                .append(alloc.keyword("lea"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(alloc.keyword("rel"))
                .append(alloc.space())
                .append(l)
                .append("]"),
            MOV(register, register1) => alloc
                .text(INDENT)
                .append(alloc.keyword("mov"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            MOVS(register, register1, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("mov"))
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(register.print(cfg, alloc)),
            MOVL(register, register1, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("mov"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]"),
            MOVI(register, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("mov"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc)),
            MOVIM(register, i1, i2) => alloc
                .text(INDENT)
                .append(alloc.keyword("mov qword"))
                .append(alloc.space())
                .append("[")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i1.print(cfg, alloc))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(i2.print(cfg, alloc)),
            CMP(register, register1) => alloc
                .text(INDENT)
                .append(alloc.keyword("cmp"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            CMPRM(register, register1, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("cmp"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(i.print(cfg, alloc))
                .append("]"),
            CMPMR(register, i, register1) => alloc
                .text(INDENT)
                .append(alloc.keyword("cmp"))
                .append(alloc.space())
                .append("[")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            CMPI(register, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("cmp"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(i.print(cfg, alloc)),
            CMPIM(register, i1, i2) => alloc
                .text(INDENT)
                .append(alloc.keyword("cmp qword"))
                .append(alloc.space())
                .append("[")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(i1.print(cfg, alloc))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(i2.print(cfg, alloc)),
            JEL(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("je"))
                .append(alloc.space())
                .append(l),
            JNEL(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("jne"))
                .append(alloc.space())
                .append(l),
            JLL(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("jl"))
                .append(alloc.space())
                .append(l),
            JLEL(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("jle"))
                .append(alloc.space())
                .append(l),
            PUSH(r) => alloc
                .text(INDENT)
                .append(alloc.keyword("push"))
                .append(alloc.space())
                .append(r.print(cfg, alloc)),
            POP(r) => alloc
                .text(INDENT)
                .append(alloc.keyword("pop"))
                .append(alloc.space())
                .append(r.print(cfg, alloc)),
            CALL(fun) => alloc
                .text(INDENT)
                .append(alloc.keyword("call"))
                .append(alloc.space())
                .append(fun),
            RET => alloc.text(INDENT).append(alloc.keyword("ret")),
            LAB(l) => alloc.hardline().append(l).append(COLON),
            NOEXECSTACK => alloc.keyword("section .note.GNU-stack noalloc noexec nowrite progbits"),
            TEXT => alloc.keyword("section .text"),
            GLOBAL(l) => alloc.keyword("global").append(alloc.space()).append(l),
            EXTERN(fun) => alloc.keyword("extern").append(alloc.space()).append(fun),
            COMMENT(msg) => alloc
                .text(INDENT)
                .append(alloc.comment(&format!("; {msg}"))),
        }
    }
}

pub fn move_from_register(temporary: Temporary, register: Register, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(target_register) => {
            instructions.push(Code::MOV(target_register, register));
        }
        Temporary::Spill(target_position) => {
            instructions.push(Code::MOVS(register, STACK, stack_offset(target_position)));
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
            instructions.push(Code::ADDRM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn add_to_spill(position: Spill, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::ADDMR(STACK, stack_offset(position), source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(source_position)));
            instructions.push(Code::ADDMR(STACK, stack_offset(position), TEMP));
        }
    }
}

pub fn mul_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::IMUL(register, source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::IMULRM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn mul_to_spill(position: Spill, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::IMULMR(STACK, stack_offset(position), source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(source_position)));
            instructions.push(Code::IMULMR(STACK, stack_offset(position), TEMP));
        }
    }
}

pub fn op_commutative(
    op_to_register: fn(register: Register, temporary: Temporary, instructions: &mut Vec<Code>),
    op_to_spill: fn(position: Spill, temporary: Temporary, instructions: &mut Vec<Code>),
    target_temporary: Temporary,
    source_temporary_1: Temporary,
    source_temporary_2: Temporary,
    instructions: &mut Vec<Code>,
) {
    match target_temporary {
        Temporary::Register(target_register) => {
            if target_temporary != source_temporary_1 {
                if target_temporary != source_temporary_2 {
                    move_to_register(target_register, source_temporary_1, instructions);
                    op_to_register(target_register, source_temporary_2, instructions);
                } else {
                    op_to_register(target_register, source_temporary_1, instructions);
                }
            } else {
                op_to_register(target_register, source_temporary_2, instructions);
            }
        }
        Temporary::Spill(target_position) => {
            if target_temporary != source_temporary_1 {
                if target_temporary != source_temporary_2 {
                    move_to_register(TEMP, source_temporary_1, instructions);
                    op_to_register(TEMP, source_temporary_2, instructions);
                    instructions.push(Code::MOVS(TEMP, STACK, stack_offset(target_position)));
                } else {
                    op_to_spill(target_position, source_temporary_1, instructions);
                }
            } else {
                op_to_spill(target_position, source_temporary_2, instructions);
            }
        }
    }
}

pub fn sub_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::SUB(register, source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::SUBRM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn sub_to_spill(position: Spill, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::SUBMR(STACK, stack_offset(position), source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(source_position)));
            instructions.push(Code::SUBMR(STACK, stack_offset(position), TEMP));
        }
    }
}

pub fn sub(
    target_temporary: Temporary,
    source_temporary_1: Temporary,
    source_temporary_2: Temporary,
    instructions: &mut Vec<Code>,
) {
    match target_temporary {
        Temporary::Register(target_register) => {
            if target_temporary != source_temporary_1 {
                if target_temporary != source_temporary_2 {
                    move_to_register(target_register, source_temporary_1, instructions);
                    sub_to_register(target_register, source_temporary_2, instructions);
                } else {
                    move_to_register(TEMP, source_temporary_1, instructions);
                    sub_to_register(TEMP, source_temporary_2, instructions);
                    instructions.push(Code::MOV(target_register, TEMP));
                }
            } else {
                sub_to_register(target_register, source_temporary_2, instructions);
            }
        }
        Temporary::Spill(target_position) => {
            if target_temporary != source_temporary_1 {
                move_to_register(TEMP, source_temporary_1, instructions);
                sub_to_register(TEMP, source_temporary_2, instructions);
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(target_position)));
            } else {
                sub_to_spill(target_position, source_temporary_2, instructions);
            }
        }
    }
}

/// Assumes that `RETURN2` is backed up in `TEMP`.
pub fn div(divisor: Temporary, instructions: &mut Vec<Code>) {
    match divisor {
        Temporary::Register(register) => {
            if register == RETURN2 {
                instructions.push(Code::CQO);
                instructions.push(Code::IDIV(TEMP));
            } else {
                instructions.push(Code::CQO);
                instructions.push(Code::IDIV(register));
            }
        }
        Temporary::Spill(position) => {
            instructions.push(Code::CQO);
            instructions.push(Code::IDIVM(STACK, stack_offset(position)));
        }
    }
}

pub fn compare(fst: Temporary, snd: Temporary, instructions: &mut Vec<Code>) {
    match (fst, snd) {
        (Temporary::Register(register_fst), Temporary::Register(register_snd)) => {
            instructions.push(Code::CMP(register_fst, register_snd));
        }
        (Temporary::Register(register_fst), Temporary::Spill(position_snd)) => {
            instructions.push(Code::CMPRM(register_fst, STACK, stack_offset(position_snd)));
        }
        (Temporary::Spill(position_fst), Temporary::Register(register_snd)) => {
            instructions.push(Code::CMPMR(STACK, stack_offset(position_fst), register_snd));
        }
        (Temporary::Spill(position_fst), Temporary::Spill(position_snd)) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position_fst)));
            instructions.push(Code::CMPRM(TEMP, STACK, stack_offset(position_snd)));
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

fn caller_save_registers_info(context: &[ContextBinding]) -> (usize, Vec<usize>) {
    let first_backup_register = std::cmp::max(2 * context.len() + RESERVED, CALLER_SAVE_LAST + 1);

    let caller_save_count = CALLER_SAVE_LAST + 1 - CALLER_SAVE_FIRST;
    let mut registers_to_save = Vec::with_capacity(caller_save_count);
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

fn save_caller_save_registers(
    first_backup_register: usize,
    registers_to_save: &[usize],
    instructions: &mut Vec<Code>,
) {
    let registers_to_save_count = registers_to_save.len();
    let backup_register_count = REGISTER_NUM.saturating_sub(first_backup_register);
    let backup_registers_used = std::cmp::min(registers_to_save_count, backup_register_count);

    for (offset, register) in registers_to_save
        .iter()
        .take(backup_registers_used)
        .enumerate()
    {
        instructions.push(Code::MOV(
            (first_backup_register + offset).into(),
            (*register).into(),
        ));
    }

    for register in registers_to_save.iter().skip(backup_registers_used) {
        instructions.push(Code::PUSH((*register).into()));
    }

    // ensure stack pointer alignment
    if (registers_to_save_count - backup_registers_used) % 2 == 0 {
        instructions.push(Code::SUBI(STACK, address(1).into()));
    }
}

fn restore_caller_save_registers(
    first_backup_register: usize,
    registers_to_save: &[usize],
    instructions: &mut Vec<Code>,
) {
    let registers_to_save_count = registers_to_save.len();
    let backup_register_count = REGISTER_NUM.saturating_sub(first_backup_register);
    let backup_registers_used = std::cmp::min(registers_to_save_count, backup_register_count);

    for (offset, register) in registers_to_save
        .iter()
        .take(backup_registers_used)
        .enumerate()
    {
        instructions.push(Code::MOV(
            (*register).into(),
            (first_backup_register + offset).into(),
        ));
    }

    if (registers_to_save_count - backup_registers_used) % 2 == 0 {
        instructions.push(Code::ADDI(STACK, address(1).into()));
    }

    for register in registers_to_save.iter().skip(backup_registers_used).rev() {
        instructions.push(Code::POP((*register).into()));
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
            Temporary::Register(register) => instructions.push(Code::JMP(register)),
            Temporary::Spill(position) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
                instructions.push(Code::JMP(TEMP));
            }
        }
    }

    fn jump_label(name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::JMPL(name));
    }

    fn jump_label_fixed(name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::JMPLN(name));
    }

    fn jump_label_if_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::JEL(name));
    }

    fn jump_label_if_not_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::JNEL(name));
    }

    fn jump_label_if_less(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::JLL(name));
    }

    fn jump_label_if_less_or_equal(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::JLEL(name));
    }

    fn jump_label_if_zero(temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::JEL(name));
    }

    fn jump_label_if_not_zero(temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::JNEL(name));
    }

    fn load_immediate(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::MOVI(register, immediate)),
            Temporary::Spill(position) => {
                instructions.push(Code::MOVIM(STACK, stack_offset(position), immediate));
            }
        }
    }

    fn load_label(temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::LEAL(register, name)),
            Temporary::Spill(position) => {
                instructions.push(Code::LEAL(TEMP, name));
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(position)));
            }
        }
    }

    fn add_and_jump(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
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
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        op_commutative(
            add_to_register,
            add_to_spill,
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
        sub(
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
        op_commutative(
            mul_to_register,
            mul_to_spill,
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
        instructions.push(Code::MOV(TEMP, RETURN2));
        move_from_register(target_temporary, RETURN1, instructions);
        move_to_register(RETURN1, source_temporary_1, instructions);
        div(source_temporary_2, instructions);
        instructions.push(Code::MOV(RETURN2, RETURN1));
        move_to_register(RETURN1, target_temporary, instructions);
        move_from_register(target_temporary, RETURN2, instructions);
        instructions.push(Code::MOV(RETURN2, TEMP));
    }

    fn rem(
        target_temporary: Temporary,
        source_temporary_1: Temporary,
        source_temporary_2: Temporary,
        instructions: &mut Vec<Code>,
    ) {
        instructions.push(Code::MOV(TEMP, RETURN2));
        move_from_register(target_temporary, RETURN1, instructions);
        move_to_register(RETURN1, source_temporary_1, instructions);
        div(source_temporary_2, instructions);
        move_to_register(RETURN1, target_temporary, instructions);
        move_from_register(target_temporary, RETURN2, instructions);
        instructions.push(Code::MOV(RETURN2, TEMP));
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
                instructions.push(Code::MOV(arg(0), source_register))
            }
            Temporary::Spill(_) => instructions.push(Code::MOV(arg(0), TEMP)),
        }
        instructions.push(Code::CALL(print_i64.to_string()));
        instructions.push(Code::COMMENT("#restore caller-save registers".to_string()));
        restore_caller_save_registers(first_backup_register, &registers_to_save, instructions);
    }
}
