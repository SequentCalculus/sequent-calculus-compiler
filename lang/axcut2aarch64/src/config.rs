use super::Backend;

use axcut2backend::config::{Config, TemporaryNumber};
use printer::{DocAllocator, Print};

use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Register {
    /// General Purpose 64-Bit register
    X(usize),
    /// Stack Pointer
    SP,
}

impl Print for Register {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Register::X(r) => alloc.text("X").append(format!("{}", r)),
            Register::SP => alloc.text("SP"),
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::X(r) => write!(f, "X{}", r),
            Register::SP => write!(f, "SP"),
        }
    }
}

impl From<usize> for Register {
    fn from(value: usize) -> Self {
        Register::X(value)
    }
}

// there can be at most 14 variables in the environment, which can be alleviated by implementing
// spilling
pub const REGISTER_NUM: usize = 31;

pub type Immediate = i64;

// x2 is used for our purposes
// x0 is a heap pointer to an object which we can directly overwrite AND the first part of the return value
// x1 is a deferred-free-list pointer to objects which we have to free AND the second part of the return value
pub const RESERVED: usize = 3;

pub const TEMP: Register = Register::X(2);
pub const HEAP: Register = Register::X(0);
pub const FREE: Register = Register::X(1);

pub const RETURN1: Register = Register::X(0);
pub const RETURN2: Register = Register::X(1);

// the size of the memory is hardcoded and can be adapted via `heapsize` in
// `infrastructure/aarch_64/driver*.c`
#[must_use]
pub const fn address(n: i64) -> i64 {
    8 * n
}

pub const FIELDS_PER_BLOCK: usize = 3;

pub const REFERENCE_COUNT_OFFSET: i64 = address(0);

pub const NEXT_ELEMENT_OFFSET: i64 = address(0);

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub const fn field_offset(number: TemporaryNumber, i: usize) -> i64 {
    address(2 + 2 * i as i64 + number as i64)
}

impl Config<Register, Immediate> for Backend {
    fn i64_to_immediate(number: i64) -> Immediate {
        number
    }

    fn temp() -> Register {
        TEMP
    }

    fn heap() -> Register {
        HEAP
    }

    fn free() -> Register {
        FREE
    }

    fn return1() -> Register {
        RETURN1
    }

    fn return2() -> Register {
        RETURN2
    }

    #[allow(clippy::cast_possible_wrap)]
    fn jump_length(n: usize) -> Immediate {
        4 * n as Immediate
    }
}
