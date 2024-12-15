use super::config::{stack_offset, Immediate, Register, Temporary, RETURN1, RETURN2, STACK, TEMP};
use super::Backend;

use axcut::syntax::Name;
use axcut2backend::code::Instructions;
use printer::theme::ThemeExt;
use printer::tokens::{COLON, COMMA, MINUS, PLUS};
use printer::{DocAllocator, Print};

/// x86-64 Assembly instructions
#[derive(Debug, Clone)]
pub enum Code {
    /// https://www.felixcloutier.com/x86/add
    ADD(Register, Register),
    /// https://www.felixcloutier.com/x86/add
    ADDM(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/add
    ADDI(Register, Immediate),
    /// https://www.felixcloutier.com/x86/add
    ADDIM(Register, Immediate, Immediate),
    /// https://www.felixcloutier.com/x86/sub
    SUB(Register, Register),
    /// https://www.felixcloutier.com/x86/sub
    SUBM(Register, Register, Immediate),
    /// https://www.felixcloutier.com/x86/sub
    SUBI(Register, Immediate),
    /// https://www.felixcloutier.com/x86/imul
    IMUL(Register, Register),
    /// https://www.felixcloutier.com/x86/imul
    IMULM(Register, Register, Immediate),
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
    JLTL(String),
    PUSH(Register),
    POP(Register),
    RET,
    LAB(String),
    TEXT,
    GLOBAL(String),
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
            ADDM(register, register1, i) => alloc
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
            SUBM(register, register1, i) => alloc
                .text(INDENT)
                .append(alloc.keyword("sub"))
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(MINUS)
                .append(alloc.space())
                .append(i.print(cfg, alloc))
                .append("]"),
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
            IMULM(register, register1, i) => alloc
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
            JLTL(l) => alloc
                .text(INDENT)
                .append(alloc.keyword("jl"))
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
            RET => alloc.text(INDENT).append(alloc.keyword("ret")),
            LAB(l) => alloc.hardline().append(l).append(COLON),
            TEXT => alloc.keyword("segment .text"),
            GLOBAL(l) => alloc.keyword("global").append(alloc.space()).append(l),
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
            instructions.push(Code::ADDM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn sub_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::SUB(register, source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::SUBM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn mul_to_register(register: Register, temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(source_register) => {
            instructions.push(Code::IMUL(register, source_register));
        }
        Temporary::Spill(source_position) => {
            instructions.push(Code::IMULM(register, STACK, stack_offset(source_position)));
        }
    }
}

pub fn op(
    op_to_register: fn(register: Register, temporary: Temporary, instructions: &mut Vec<Code>),
    target_temporary: Temporary,
    source_temporary_1: Temporary,
    source_temporary_2: Temporary,
    instructions: &mut Vec<Code>,
) {
    match target_temporary {
        Temporary::Register(target_register) => {
            if target_temporary != source_temporary_1 {
                move_to_register(target_register, source_temporary_1, instructions);
            }
            op_to_register(target_register, source_temporary_2, instructions);
        }
        Temporary::Spill(target_position) => {
            move_to_register(TEMP, source_temporary_1, instructions);
            op_to_register(TEMP, source_temporary_2, instructions);
            instructions.push(Code::MOVS(TEMP, STACK, stack_offset(target_position)));
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

    fn jump_label_if_less(
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::JLTL(name));
    }

    fn jump_label_if_zero(temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, Immediate { val: 0 }, instructions);
        instructions.push(Code::JEL(name));
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
        op(
            add_to_register,
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
            sub_to_register,
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
            mul_to_register,
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
        match (source_temporary, target_temporary) {
            (Temporary::Register(source_register), Temporary::Register(target_register)) => {
                instructions.push(Code::MOV(target_register, source_register));
            }
            (Temporary::Register(source_register), Temporary::Spill(target_position)) => {
                instructions.push(Code::MOVS(
                    source_register,
                    STACK,
                    stack_offset(target_position),
                ));
            }
            (Temporary::Spill(source_position), Temporary::Register(target_register)) => {
                instructions.push(Code::MOVL(
                    target_register,
                    STACK,
                    stack_offset(source_position),
                ));
            }
            (Temporary::Spill(source_position), Temporary::Spill(target_position)) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(source_position)));
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(target_position)));
            }
        }
    }
}
