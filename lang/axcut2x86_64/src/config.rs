use axcut::syntax::{TypingContext, Var};

use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Register(pub usize);

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register(0) => write!(f, "rsp"),
            Register(1) => write!(f, "rcx"),
            Register(2) => write!(f, "rbx"),
            Register(3) => write!(f, "rbp"),
            Register(4) => write!(f, "rax"),
            Register(5) => write!(f, "rdx"),
            Register(6) => write!(f, "rsi"),
            Register(7) => write!(f, "rdi"),
            Register(n) => write!(f, "r{n}"),
        }
    }
}

impl From<usize> for Register {
    fn from(value: usize) -> Self {
        Register(value)
    }
}

pub const REGISTER_NUM: usize = 16;

pub type Immediate = i64;

// rsp is the stack pointer we use for register spills
// rcx is used for our purposes
// rbx is a heap pointer to an object which we can directly overwrite
// rbp is a deferred-free-list pointer to objects which we have to free

pub const RESERVED: usize = 4;

pub const STACK: Register = Register(0);
pub const TEMP: Register = Register(1);
pub const HEAP: Register = Register(2);
pub const FREE: Register = Register(3);

pub const RETURN1: Register = Register(4);
pub const RETURN2: Register = Register(5);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Spill(pub usize);

pub const SPILL_NUM: usize = 256;
#[allow(clippy::cast_possible_wrap)]
pub const SPILL_SPACE: i64 = SPILL_NUM as i64 * 8;

// one spot is used for our purposes

pub const RESERVED_SPILLS: usize = 1;

pub const SPILL_TEMP: Spill = Spill(0);

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub const fn stack_offset(position: Spill) -> i64 {
    SPILL_SPACE - (8 * (position.0 as i64 + 1))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Temporary {
    R(Register),
    S(Spill),
}

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub const fn jump_length(n: usize) -> i64 {
    2 * n as i64
}

#[must_use]
pub const fn address(n: i64) -> i64 {
    8 * n
}

pub const FIELDS_PER_BLOCK: usize = 3;

pub const REFERENCE_COUNT_OFFSET: i64 = address(0);

pub const NEXT_ELEMENT_OFFSET: i64 = address(0);

#[derive(Copy, Clone)]
pub enum TemporaryNumber {
    Fst = 0,
    Snd = 1,
}

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub const fn field_offset(number: TemporaryNumber, i: usize) -> i64 {
    address(2 + 2 * i as i64 + number as i64)
}

#[must_use]
pub fn arg(number: usize) -> Register {
    match number {
        0 => Register(7),
        1 => Register(6),
        2 => Register(5),
        3 => Register(10),
        4 => Register(8),
        5 => Register(9),
        _ => panic!("syscalls can have 6 arguments at most"),
    }
}

#[must_use]
pub fn variable_temporary(
    number: TemporaryNumber,
    context: &TypingContext,
    variable: &Var,
) -> Temporary {
    fn get_position(context: &TypingContext, variable: &Var) -> usize {
        context
            .iter()
            .position(|binding| binding.var == *variable)
            .unwrap_or_else(|| panic!("Variable {variable} not found in context {context:?}"))
    }

    let position = 2 * get_position(context, variable) + number as usize;
    let register_number = position + RESERVED;
    if register_number < REGISTER_NUM {
        Temporary::R(Register(register_number))
    } else {
        let spill_number = register_number - REGISTER_NUM + RESERVED_SPILLS;
        assert!(spill_number < SPILL_NUM, "Out of temporaries");
        Temporary::S(Spill(spill_number))
    }
}

#[must_use]
pub fn fresh_temporary(number: TemporaryNumber, context: &TypingContext) -> Temporary {
    let position = 2 * context.len() + number as usize;
    let register_number = position + RESERVED;
    if register_number < REGISTER_NUM {
        Temporary::R(Register(register_number))
    } else {
        let spill_number = register_number - REGISTER_NUM + RESERVED_SPILLS;
        assert!(spill_number < SPILL_NUM, "Out of temporaries");
        Temporary::S(Spill(spill_number))
    }
}
