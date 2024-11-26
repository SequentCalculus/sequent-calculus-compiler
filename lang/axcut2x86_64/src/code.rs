use super::config::{stack_offset, Immediate, Register, Temporary, RETURN1, RETURN2, STACK, TEMP};
use super::Backend;

use axcut::syntax::Name;
use axcut2backend::code::Instructions;
use printer::theme::ThemeExt;
use printer::tokens::{COLON, COMMA, MINUS, PLUS};
use printer::{DocAllocator, Print};

use std::fmt::{self};

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
    LAB(String),
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Code::ADD(x, y) => write!(f, "add {x}, {y}"),
            Code::ADDM(x, y, c) => write!(f, "add {x}, [{y} + {c}]"),
            Code::ADDI(x, c) => write!(f, "add {x}, {c}"),
            Code::ADDIM(x, c, d) => write!(f, "add qword [{x} + {c}], {d}"),
            Code::SUB(x, y) => write!(f, "sub {x}, {y}"),
            Code::SUBM(x, y, c) => write!(f, "sub {x}, [{y} + {c}]"),
            Code::IMUL(x, y) => write!(f, "imul {x}, {y}"),
            Code::IMULM(x, y, c) => write!(f, "imul {x}, [{y} + {c}]"),
            Code::IDIV(x) => write!(f, "idiv {x}"),
            Code::IDIVM(x, c) => write!(f, "idiv qword [{x} + {c}]"),
            Code::CQO => write!(f, "cqo"),
            Code::JMP(x) => write!(f, "jmp {x}"),
            Code::JMPL(l) => write!(f, "jmp {l}"),
            Code::LEAL(x, l) => write!(f, "lea {x}, [rel {l}]"),
            Code::MOV(x, y) => write!(f, "mov {x}, {y}"),
            // `MOVS` has different order of arguments!
            Code::MOVS(x, y, c) => write!(f, "mov [{y} + {c}], {x}"),
            Code::MOVL(x, y, c) => write!(f, "mov {x}, [{y} + {c}]"),
            Code::MOVI(x, c) => write!(f, "mov {x}, {c}"),
            Code::MOVIM(x, c, d) => write!(f, "mov qword [{x} + {c}], {d}"),
            Code::CMP(x, y) => write!(f, "cmp {x}, {y}"),
            Code::CMPRM(x, y, c) => write!(f, "cmp {x}, [{y} + {c}]"),
            Code::CMPMR(x, c, y) => write!(f, "cmp [{x} + {c}], {y}"),
            Code::CMPI(x, c) => write!(f, "cmp {x}, {c}"),
            Code::CMPIM(x, c, d) => write!(f, "cmp qword [{x} + {c}], {d}"),
            Code::JEL(l) => write!(f, "je {l}"),
            Code::JLTL(l) => write!(f, "jl {l}"),
            Code::LAB(l) => write!(f, "\n{l}:"),
        }
    }
}

impl Print for Code {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Code::ADD(register, register1) => alloc
                .keyword("add")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            Code::ADDM(register, register1, i) => alloc
                .keyword("add")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(format!("{i}"))
                .append("]"),
            Code::ADDI(register, i) => alloc
                .keyword("add")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{i}")),
            Code::ADDIM(register, i1, i2) => alloc
                .keyword("add qword [")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(format! {"{i1}"})
                .append("]")
                .append(alloc.space())
                .append(format!("{i2}")),
            Code::SUB(register, register1) => alloc
                .keyword("sub")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            Code::SUBM(register, register1, i) => alloc
                .keyword("sub")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(MINUS)
                .append(alloc.space())
                .append(format!("{i}"))
                .append("]"),
            Code::IMUL(register, register1) => alloc
                .keyword("imul")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            Code::IMULM(register, register1, i) => alloc
                .keyword("imul")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(MINUS)
                .append(alloc.space())
                .append(format!("{i}"))
                .append("]"),
            Code::IDIV(register) => alloc
                .keyword("idiv")
                .append(alloc.space())
                .append(register.print(cfg, alloc)),
            Code::IDIVM(register, i) => alloc
                .keyword("idiv qword")
                .append(alloc.space())
                .append("[")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(format!("{i}"))
                .append("]"),
            Code::CQO => alloc.keyword("cqo"),
            Code::JMP(register) => alloc
                .keyword("jmp")
                .append(alloc.space())
                .append(register.print(cfg, alloc)),
            Code::JMPL(l) => alloc.keyword("jmp").append(alloc.space()).append(l),
            Code::LEAL(register, l) => alloc
                .keyword("lea")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(alloc.keyword("rel"))
                .append(alloc.space())
                .append(l)
                .append("]"),
            Code::MOV(register, register1) => alloc
                .keyword("mov")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            Code::MOVS(register, register1, i) => alloc
                .keyword("mov")
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(format!("{i}"))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(register.print(cfg, alloc)),
            Code::MOVL(register, register1, i) => alloc
                .keyword("mov")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(format!("{i}"))
                .append("]"),
            Code::MOVI(register, i) => alloc
                .keyword("mov")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{i}")),
            Code::MOVIM(register, i1, i2) => alloc
                .keyword("mov qword")
                .append(alloc.space())
                .append("[")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(format!("{i1}"))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{i2}")),
            Code::CMP(register, register1) => alloc
                .keyword("cmp")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            Code::CMPRM(register, register1, i) => alloc
                .keyword("cmp")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append("[")
                .append(register1.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(format!("{i}"))
                .append("]"),
            Code::CMPMR(register, i, register1) => alloc
                .keyword("cmp")
                .append(alloc.space())
                .append("[")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(format!("{i}"))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(register1.print(cfg, alloc)),
            Code::CMPI(register, i) => alloc
                .keyword("cmp")
                .append(alloc.space())
                .append(register.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{i}")),
            Code::CMPIM(register, i1, i2) => alloc
                .keyword("cmp qword")
                .append(alloc.space())
                .append("[")
                .append(register.print(cfg, alloc))
                .append(alloc.space())
                .append(PLUS)
                .append(alloc.space())
                .append(format!("{i1}"))
                .append("]")
                .append(COMMA)
                .append(alloc.space())
                .append(format!("{i2}")),
            Code::JEL(l) => alloc.keyword("je").append(alloc.space()).append(l),
            Code::JLTL(l) => alloc.keyword("jl").append(alloc.space()).append(l),
            Code::LAB(l) => alloc.hardline().append(l).append(COLON),
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
            instructions.push(Code::CMP(register_fst, register_snd))
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
    fn label(&self, name: Name) -> Code {
        Code::LAB(name)
    }

    fn jump(&self, temporary: Temporary, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::JMP(register)),
            Temporary::Spill(position) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
                instructions.push(Code::JMP(TEMP));
            }
        }
    }

    fn jump_label(&self, name: Name, instructions: &mut Vec<Code>) {
        instructions.push(Code::JMPL(name));
    }

    fn jump_label_if_equal(
        &self,
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::JEL(name));
    }

    fn jump_label_if_less(
        &self,
        fst: Temporary,
        snd: Temporary,
        name: Name,
        instructions: &mut Vec<Code>,
    ) {
        compare(fst, snd, instructions);
        instructions.push(Code::JLTL(name));
    }

    fn jump_label_if_zero(&self, temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        compare_immediate(temporary, 0, instructions);
        instructions.push(Code::JEL(name));
    }

    fn load_immediate(
        &self,
        temporary: Temporary,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    ) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::MOVI(register, immediate)),
            Temporary::Spill(position) => {
                instructions.push(Code::MOVIM(STACK, stack_offset(position), immediate));
            }
        }
    }

    fn load_label(&self, temporary: Temporary, name: Name, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => instructions.push(Code::LEAL(register, name)),
            Temporary::Spill(position) => {
                instructions.push(Code::LEAL(TEMP, name));
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(position)));
            }
        }
    }

    fn add_and_jump(
        &self,
        temporary: Temporary,
        immediate: Immediate,
        instructions: &mut Vec<Code>,
    ) {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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

    fn mov(
        &self,
        target_temporary: Temporary,
        source_temporary: Temporary,
        instructions: &mut Vec<Code>,
    ) {
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
