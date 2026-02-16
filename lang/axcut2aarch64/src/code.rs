//! This module implements the abstract methods for machine instructions.

use super::Backend;
use super::config::{
    CALLER_SAVE_FIRST, CALLER_SAVE_LAST, Immediate, REGISTER_NUM, RESERVED, Register, SPILL_TEMP,
    TEMP, TEMP2, TEMPORARY_TEMP, Temporary, address, stack_offset,
};

use axcut::syntax::{Chirality, ContextBinding};
use axcut2backend::code::Instructions;
use printer::theme::ThemeExt;
use printer::tokens::{COLON, COMMA, PRINT_I64, PRINTLN_I64};
use printer::{DocAllocator, Print};

/// This enum provides the concrete machine instructions. Each variant stands either for one
/// instruction or for a label, a comment or a control directive.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Code {
    /// [Link to documentation.](<https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/ADD--shifted-register---Add-optionally-shifted-register-?lang=en>)
    ADD(Register, Register, Register),
    /// This instruction assumes that the immediate is in the range `0` to `4095`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/ADD--immediate---Add-immediate-value-?lang=en)
    ADDI(Register, Register, Immediate),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/SUB--shifted-register---Subtract-optionally-shifted-register-?lang=en)
    SUB(Register, Register, Register),
    /// This instruction assumes that the immediate is in the range `0` to `4095`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/SUB--immediate---Subtract-immediate-value-?lang=en)
    SUBI(Register, Register, Immediate),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/MUL--Multiply--an-alias-of-MADD-?lang=en)
    MUL(Register, Register, Register),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/SDIV--Signed-divide-?lang=en)
    SDIV(Register, Register, Register),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/MSUB--Multiply-subtract-?lang=en)
    MSUB(Register, Register, Register, Register),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/B--Branch-?lang=en)
    B(String),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/BR--Branch-to-register-?lang=en)
    BR(Register),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/BL--Branch-with-link-?lang=en)
    BL(String),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/ADR--Form-PC-relative-address-?lang=en)
    ADR(Register, String),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/MOV--register---Move-register-value--an-alias-of-ORR--shifted-register--?lang=en)
    MOVR(Register, Register),
    /// This instruction assumes that the first immediate is in the range `0` to `65535` and the
    /// second immediate is a shift which can be `0`, `16`, `32`, or `48`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en)
    MOVZ(Register, Immediate, Immediate),
    /// This instruction assumes that the first immediate is in the range `0` to `65535` and the
    /// second immediate is a shift which can be `0`, `16`, `32`, or `48`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/MOVN--Move-wide-with-NOT-?lang=en)
    MOVN(Register, Immediate, Immediate),
    /// This instruction assumes that the first immediate is in the range `0` to `65535` and the
    /// second immediate is a shift which can be `0`, `16`, `32`, or `48`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/MOVK--Move-wide-with-keep-?lang=en)
    MOVK(Register, Immediate, Immediate),
    /// This instruction assumes that the immediate is in the range `0` to `32760` and it should be
    /// a multiple of `8`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/LDR--immediate---Load-register--immediate--?lang=en)
    LDR(Register, Register, Immediate),
    /// This instruction is only used in the cleanup code. It assumes that the immediate is in the
    /// range `-512` to `504` and it should be a multiple of `8`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/LDP--Load-pair-of-registers-)
    LDP_POST_INDEX(Register, Register, Register, Immediate),
    /// This instruction assumes that the immediate is in the range `0` to `32760` and it should be
    /// a multiple of `8`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/STR--immediate---Store-register--immediate--)
    STR(Register, Register, Immediate),
    /// This instruction is only used in the setup code. It assumes that the immediate is in the
    /// range `-512` to `504` and it should be a multiple of `8`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/STP--Store-pair-of-registers-)
    STP_PRE_INDEX(Register, Register, Register, Immediate),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/CMP--shifted-register---Compare--shifted-register---an-alias-of-SUBS--shifted-register--)
    CMPR(Register, Register),
    /// This instruction assumes that the immediate is in the range `0` to `4095`.
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/CMP--immediate---Compare--immediate---an-alias-of-SUBS--immediate--)
    CMPI(Register, Immediate),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/B-cond--Branch-conditionally-)
    BEQ(String),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/B-cond--Branch-conditionally-)
    BNE(String),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/B-cond--Branch-conditionally-)
    BLT(String),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/B-cond--Branch-conditionally-)
    BLE(String),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/B-cond--Branch-conditionally-)
    BGT(String),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/B-cond--Branch-conditionally-)
    BGE(String),
    /// [Link to documentation.](https://developer.arm.com/documentation/ddi0602/2025-03/Base-Instructions/RET--Return-from-subroutine-)
    RET,
    /// An assembly label.
    LAB(String),
    /// Marks the start of the text segment.
    TEXT,
    /// Marks its argument as global routine.
    GLOBAL(String),
    /// An assembly comment.
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
            BGT(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("BGT"))
                .append(alloc.space())
                .append(l),
            BGE(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("BGE"))
                .append(alloc.space())
                .append(l),
            RET => alloc.text(INDENT).append(alloc.keyword("RET")),
            LAB(l) => alloc.hardline().append(l).append(COLON),
            TEXT => alloc.keyword(".text"),
            GLOBAL(l) => alloc.keyword(".global").append(alloc.space()).append(l),
            COMMENT(msg) => alloc
                .text(INDENT)
                .append(alloc.comment(&format!("// {msg}"))),
        }
    }
}

/// This function generates code for moving the contents of a register into a temporary.
fn move_from_register(temporary: Temporary, register: Register, instructions: &mut Vec<Code>) {
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

/// This function generates code for moving the contents of a temporary into a register.
fn move_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
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

/// This function generates code for adding the two source registers.
fn add(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::ADD(target, source_1, source_2));
}

/// This function generates code for subtracting the second source register from the first one.
fn sub(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::SUB(target, source_1, source_2));
}

/// This function generates code for multiplying the two source registers.
fn mul(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::MUL(target, source_1, source_2));
}

/// This function generates code for dividing the first source register by the second one.
fn div(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
    instructions.push(Code::SDIV(target, source_1, source_2));
}

/// This function generates code for calculating the remainder when dividing the first source
/// register by the second one. It assumes that if the second source register is the second scratch
/// register [`super::config::TEMP2`], then the first source register must be the first scratch
/// register [`super::config::TEMP`].
fn rem(target: Register, source_1: Register, source_2: Register, instructions: &mut Vec<Code>) {
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

/// This function generates code for arithmetic operations. It distinguishes between register
/// operands and operands in spill positions.
/// - `op` is the operation to perform on register operands.
/// - `target_temporary` is the temporary for the result.
/// - `source_temporary_1` is the first source temporary.
/// - `source_temporary_2` is the second source temporary.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn op(
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

/// This function generates code for comparing the contents of a temporary and a register.
fn compare(fst: Temporary, snd: Temporary, instructions: &mut Vec<Code>) {
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

/// This function generates code for comparing the contents of a temporary and an immediate. It
/// assumes that the immediate is in the range `0` to `4095`.
fn compare_immediate(temporary: Temporary, immediate: Immediate, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(register) => instructions.push(Code::CMPI(register, immediate)),
        Temporary::Spill(position) => {
            instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position)));
            instructions.push(Code::CMPI(TEMP, immediate));
        }
    }
}

/// This function calculates information for adhering to the calling convention for calling C
/// functions based on the current typing context. It returns the first register which can be used
/// for evacuating registers needed during the function call and a list of the registers that have
/// to be evacuated.
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
        // values of external types like integers occupy only one register
        if binding.chi == Chirality::Ext {
            registers_to_save.push(CALLER_SAVE_FIRST + 2 * offset + 1);
        } else {
            registers_to_save.push(CALLER_SAVE_FIRST + 2 * offset);
            registers_to_save.push(CALLER_SAVE_FIRST + 2 * offset + 1);
        }
    }

    (first_backup_register, registers_to_save)
}

/// This function generates code for for evacuating registers needed during a function call
/// adhering to the calling convention for C.
/// - `first_backup_register` is the first register which can be used for evacuating registers.
/// - `registers_to_save` is a list of the registers that have to be evacuated.
/// - `instructions` is the list of instructions to which the new instructions are appended.
#[allow(clippy::cast_possible_wrap)]
fn save_caller_save_registers(
    first_backup_register: usize,
    registers_to_save: &[usize],
    instructions: &mut Vec<Code>,
) {
    let registers_to_save_count = registers_to_save.len();
    // the last register will contain the return address
    let backup_register_count = (REGISTER_NUM - 1).saturating_sub(first_backup_register);
    let backup_registers_used = std::cmp::min(registers_to_save_count, backup_register_count);

    // we evacuate as many registers as possible into free registers
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

    // the other registers are evacuated to the stack
    let mut registers_to_push_count = registers_to_save_count - backup_registers_used;
    if registers_to_push_count > 0 {
        // ensure stack pointer alignment
        if !registers_to_push_count.is_multiple_of(2) {
            registers_to_push_count += 1;
        }
        instructions.push(Code::SUBI(
            Register::SP,
            Register::SP,
            address(registers_to_push_count as isize),
        ));
        for (offset, register) in registers_to_save
            .iter()
            .skip(backup_registers_used)
            .enumerate()
        {
            instructions.push(Code::STR(
                (*register).into(),
                Register::SP,
                address((registers_to_push_count - 1 - offset) as isize),
            ));
        }
    }
}

/// This function generates code for for restoring evacuated registers needed during a function
/// call adhering to the calling convention for C.
/// - `first_backup_register` is the first register were used for evacuating registers.
/// - `registers_to_save` is a list of the registers that had been evacuated.
/// - `instructions` is the list of instructions to which the new instructions are appended.
#[allow(clippy::cast_possible_wrap)]
fn restore_caller_save_registers(
    first_backup_register: usize,
    registers_to_save: &[usize],
    instructions: &mut Vec<Code>,
) {
    let registers_to_save_count = registers_to_save.len();
    // the last register will contain the return address
    let backup_register_count = (REGISTER_NUM - 1).saturating_sub(first_backup_register);
    let backup_registers_used = std::cmp::min(registers_to_save_count, backup_register_count);

    // we had evacuated as many registers as possible into free registers
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

    // the other registers had been evacuated to the stack
    let mut registers_to_push_count = registers_to_save_count - backup_registers_used;
    if registers_to_push_count > 0 {
        // ensure stack pointer alignment
        if !registers_to_push_count.is_multiple_of(2) {
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
                address((registers_to_push_count - 1 - offset) as isize),
            ));
        }
        instructions.push(Code::ADDI(
            Register::SP,
            Register::SP,
            address(registers_to_push_count as isize),
        ));
    }
}

impl Instructions<Code, Temporary, Immediate> for Backend {
    fn comment(msg: String) -> Code {
        Code::COMMENT(msg)
    }

    fn label(name: String) -> Code {
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

    fn jump_label(name: String, instructions: &mut Vec<Code>) {
        instructions.push(Code::B(name));
    }

    fn jump_label_fixed(name: String, instructions: &mut Vec<Code>) {
        instructions.push(Code::B(name));
    }

    fn jump_label_if_equal(
        fst: Temporary,
        snd: Temporary,
        name: String,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BEQ(name));
    }

    fn jump_label_if_not_equal(
        fst: Temporary,
        snd: Temporary,
        name: String,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BNE(name));
    }

    fn jump_label_if_less(
        fst: Temporary,
        snd: Temporary,
        name: String,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BLT(name));
    }

    fn jump_label_if_less_or_equal(
        fst: Temporary,
        snd: Temporary,
        name: String,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BLE(name));
    }

    fn jump_label_if_greater(
        fst: Temporary,
        snd: Temporary,
        name: String,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BGT(name));
    }

    fn jump_label_if_greater_or_equal(
        fst: Temporary,
        snd: Temporary,
        name: String,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::BGE(name));
    }

    fn jump_label_if_zero(temporary: Temporary, name: String, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::BEQ(name));
    }

    fn jump_label_if_not_zero(temporary: Temporary, name: String, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::BNE(name));
    }

    fn jump_label_if_less_zero(temporary: Temporary, name: String, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::BLT(name));
    }

    fn jump_label_if_less_or_equal_zero(
        temporary: Temporary,
        name: String,
        instructions: &mut Vec<Code>,
    ) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::BLE(name));
    }

    fn jump_label_if_greater_zero(
        temporary: Temporary,
        name: String,
        instructions: &mut Vec<Code>,
    ) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::BGT(name));
    }

    fn jump_label_if_greater_or_equal_zero(
        temporary: Temporary,
        name: String,
        instructions: &mut Vec<Code>,
    ) {
        compare_immediate(temporary, 0.into(), instructions);
        instructions.push(Code::BGE(name));
    }

    /// This implementation for loading an immediate into a temporary takes into account that an
    /// immediate in a `MOV` instruction can only have 16 bits (a halfword). Thus we have to load
    /// the halfwords individually, but we try to minimize the instruction count.
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

    fn load_label(temporary: Temporary, name: String, instructions: &mut Vec<Code>) {
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
            // we use the second scratch register because the parallel-moves algorithm uses the
            // first one for a different purpose
            move_to_register(TEMP2, source_temporary, instructions);
            move_from_register(target_temporary, TEMP2, instructions);
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
