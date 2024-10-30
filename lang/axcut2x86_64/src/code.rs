use super::config::{Immediate, Register};

use std::fmt;

#[derive(Debug, Clone)]
pub enum Code {
    ADD(Register, Register),
    ADDM(Register, Register, Immediate),
    //ADDMR(Register, Immediate, Register),
    ADDI(Register, Immediate),
    ADDIM(Register, Immediate, Immediate),
    SUB(Register, Register),
    SUBM(Register, Register, Immediate),
    //SUBMR(Register, Immediate, Register),
    //NEG(Register),
    //NEGM(Register, Immediate),
    IMUL(Register, Register),
    IMULM(Register, Register, Immediate),
    //IDIV(Register),
    //IDIVM(Register, Immediate),
    //CQO(Immediate),
    JMP(Register),
    //JMPI(Immediate),
    JMPL(String),
    LEAL(Register, String),
    MOV(Register, Register),
    MOVS(Register, Register, Immediate),
    //MOVRB(Register, Register, Immediate),
    MOVL(Register, Register, Immediate),
    //MOVZX(Register, Register, Immediate),
    MOVI(Register, Immediate),
    MOVIM(Register, Immediate, Immediate),
    CMP(Register, Register),
    CMPRM(Register, Register, Immediate),
    CMPMR(Register, Immediate, Register),
    CMPI(Register, Immediate),
    CMPIM(Register, Immediate, Immediate),
    //JE(Immediate),
    JEL(String),
    //JLTL(String),
    LAB(String),
    //PUSH(Register),
    //POP(Register),
    //SYSCALL,
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
            Code::JMP(x) => write!(f, "jmp {x}"),
            Code::JMPL(l) => write!(f, "jmp {l}"),
            Code::LEAL(x, l) => write!(f, "lea {x}, [rel {l}]"),
            Code::MOV(x, y) => write!(f, "mov {x}, {y}"),
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
